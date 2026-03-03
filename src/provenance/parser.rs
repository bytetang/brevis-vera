//! C2PA metadata parser.
//!
//! Extracts C2PA metadata from media files using the `c2pa` crate's stable
//! `ManifestStore` API. The parsed metadata includes signature information,
//! assertions/claims, and ingredient references.
//!
//! # C2PA Standard
//!
//! C2PA (Coalition for Content Provenance and Authenticity) uses ECDSA P-256
//! for digital signatures. The actual signature verification happens inside
//! the ZKVM circuit for privacy-preserving verification — this module only
//! *extracts* the metadata; it does not verify signatures.
//!
//! # Parsing Strategy
//!
//! This module uses the `c2pa` crate's `ManifestStore::from_file` to load
//! the manifest data, then extracts fields via the typed Rust API. The full
//! manifest store is also serialized to JSON and preserved in
//! [`C2paMetadata::raw_manifest_store`] for downstream consumers.

use std::path::Path;

use super::types::{C2paMetadata, ClaimAssertion, EcdsaSignature, Ingredient, ProvenanceError, SignatureInfo};

/// Parse C2PA metadata from a media file.
///
/// Returns `Ok(None)` if the file contains no C2PA metadata.
/// Returns `Ok(Some(metadata))` if C2PA metadata was found and parsed.
///
/// # Arguments
/// * `path` - Path to a media file (JPEG or PNG)
///
/// # Returns
/// * `None` if no C2PA manifest is embedded in the file
/// * `Some(C2paMetadata)` with all extracted provenance data
///
/// # Errors
/// * [`ProvenanceError::C2paError`] if parsing fails for reasons other than
///   missing metadata
pub fn parse_c2pa(path: &Path) -> Result<Option<C2paMetadata>, ProvenanceError> {
    let store = match c2pa::ManifestStore::from_file(path) {
        Ok(s) => s,
        Err(e) => {
            // c2pa returns an error when no JUMBF/manifest is found in the file.
            // We treat this as "no metadata present" rather than a hard error.
            let err_str = e.to_string().to_lowercase();
            if err_str.contains("jumbf")
                || err_str.contains("not found")
                || err_str.contains("no manifest")
                || err_str.contains("unsupported")
            {
                return Ok(None);
            }
            return Err(ProvenanceError::C2paError(e.to_string()));
        }
    };

    // Get the active manifest
    let active_manifest_obj = match store.get_active() {
        Some(m) => m,
        None => return Ok(None),
    };

    let active_label = store.active_label().unwrap_or("unknown").to_string();

    // Serialize the full manifest store to JSON for reference
    let raw_manifest_store =
        serde_json::to_value(&store).unwrap_or(serde_json::Value::Null);

    // Extract fields from the active manifest using the typed Rust API
    let claim_generator = active_manifest_obj.claim_generator().to_string();
    let title = active_manifest_obj.title().map(String::from);
    let format = active_manifest_obj.format().to_string();

    let signature_info = extract_signature_info(active_manifest_obj, path);
    let assertions = extract_assertions(active_manifest_obj);
    let ingredients = extract_ingredients(active_manifest_obj);

    Ok(Some(C2paMetadata {
        active_manifest: active_label,
        claim_generator,
        title,
        format,
        signature_info,
        assertions,
        ingredients,
        raw_manifest_store,
    }))
}

/// Extract signature information from a C2PA [`c2pa::Manifest`].
fn extract_signature_info(manifest: &c2pa::Manifest, path: &Path) -> Option<SignatureInfo> {
    let sig = manifest.signature_info()?;

    // Try to extract ECDSA signature and public key from the manifest
    let (ecdsa_signature, public_key) = extract_ecdsa_data(manifest, path);

    Some(SignatureInfo {
        issuer: sig.issuer.clone(),
        time: sig.time.clone(),
        cert_serial_number: sig.cert_serial_number.clone(),
        alg: sig.alg.as_ref().map(|a| a.to_string()),
        ecdsa_signature,
        public_key,
    })
}

/// Extract ECDSA signature (r, s) and public key from the C2PA manifest.
///
/// Extracts the raw signature bytes by parsing the JUMBF/COSE_Sign1 structure
/// directly from the file, and the public key from the X.509 certificate chain.
fn extract_ecdsa_data(manifest: &c2pa::Manifest, path: &Path) -> (Option<EcdsaSignature>, Option<String>) {
    // Get signature info which contains certificate chain
    let sig_info = match manifest.signature_info() {
        Some(info) => info,
        None => return (None, None),
    };

    // Extract public key from the first certificate in the chain
    let cert_chain = sig_info.cert_chain();
    let public_key = if cert_chain.is_empty() {
        None
    } else {
        extract_public_key_from_pem(cert_chain)
    };

    // Extract ECDSA signature from the COSE_Sign1 structure in the file
    let ecdsa_signature = extract_signature_from_file(path);

    (ecdsa_signature, public_key)
}

/// Extract the public key from a PEM-encoded certificate chain.
///
/// Returns the public key as an uncompressed ECDSA P-256 point (04 prefix + X + Y).
fn extract_public_key_from_pem(pem_cert_chain: &str) -> Option<String> {
    // Find the first certificate in the chain (between -----BEGIN and -----END)
    let cert_start = pem_cert_chain.find("-----BEGIN CERTIFICATE-----")?;
    let cert_end = cert_start + pem_cert_chain[cert_start..].find("-----END CERTIFICATE-----")? + 25;
    let pem_cert = &pem_cert_chain[cert_start..cert_end];

    // Parse the PEM using openssl
    let cert = openssl::x509::X509::from_pem(pem_cert.as_bytes()).ok()?;

    // Get the public key from the certificate
    let pub_key = cert.public_key().ok()?;

    // Try to get the EC key structure
    let ec_key = pub_key.ec_key().ok()?;
    let ec_group = ec_key.group();
    let ec_point = ec_key.public_key();

    // Create a context for point conversion
    let mut ctx = openssl::bn::BigNumContext::new().ok()?;

    // Get the point in uncompressed form (should start with 0x04)
    let point_bytes = ec_point.to_bytes(
        ec_group,
        openssl::ec::PointConversionForm::UNCOMPRESSED,
        &mut ctx
    ).ok()?;

    // Verify it starts with 0x04 (uncompressed form)
    if point_bytes.first() != Some(&0x04) {
        return None;
    }

    // Convert to hex string
    Some(hex::encode(&point_bytes))
}

// ---------------------------------------------------------------------------
// JUMBF + COSE_Sign1 signature extraction
// ---------------------------------------------------------------------------

/// Extract ECDSA signature from a C2PA-signed file by parsing the COSE_Sign1
/// structure embedded in the JUMBF data.
///
/// The signature is stored inside the `c2pa.signature` JUMBF superbox as
/// tagged CBOR (COSE_Sign1, tag 18). For ES256 (ECDSA P-256), the signature
/// is 64 bytes in IEEE P1363 format: `r (32 bytes) || s (32 bytes)`.
fn extract_signature_from_file(path: &Path) -> Option<EcdsaSignature> {
    let data = std::fs::read(path).ok()?;
    let jumbf_data = extract_jumbf_from_jpeg(&data)?;
    let cose_cbor = find_signature_cbor_in_jumbf(&jumbf_data)?;
    parse_ecdsa_from_cose_sign1(&cose_cbor)
}

/// Extract JUMBF data from JPEG APP11 segments.
///
/// Per ITU-T T.84 Annex E, JUMBF boxes are stored in APP11 (0xFFEB) markers.
/// Multi-segment JUMBF is reassembled by grouping segments with the same
/// box instance number (En) and ordering by packet sequence number (Z).
fn extract_jumbf_from_jpeg(data: &[u8]) -> Option<Vec<u8>> {
    use std::collections::BTreeMap;

    let mut segments: Vec<(u32, u32, Vec<u8>)> = Vec::new();

    // Verify JPEG SOI marker
    if data.len() < 2 || data[0] != 0xFF || data[1] != 0xD8 {
        return None;
    }
    let mut pos: usize = 2;

    while pos + 4 <= data.len() {
        if data[pos] != 0xFF {
            pos += 1;
            continue;
        }

        // Skip padding 0xFF bytes
        while pos + 1 < data.len() && data[pos + 1] == 0xFF {
            pos += 1;
        }
        if pos + 1 >= data.len() {
            break;
        }

        let marker = data[pos + 1];
        pos += 2; // past 0xFF XX

        // End of image
        if marker == 0xD9 {
            break;
        }

        // Markers without length: SOI(D8), TEM(01), RST0-RST7(D0-D7)
        if marker == 0xD8
            || marker == 0x00
            || marker == 0x01
            || (0xD0..=0xD7).contains(&marker)
        {
            continue;
        }

        // SOS marker — all APP markers precede SOS in JPEG
        if marker == 0xDA {
            break;
        }

        // Read segment length (2 bytes BE, includes the length field itself)
        if pos + 2 > data.len() {
            break;
        }
        let seg_len = u16::from_be_bytes([data[pos], data[pos + 1]]) as usize;
        if seg_len < 2 || pos + seg_len > data.len() {
            break;
        }

        // APP11 (0xEB): JUMBF carrier
        // Overhead after Lp: CI(2) + En(2) + Z(4) = 8 bytes
        // (ISO/IEC 19566-5: En is 16-bit box instance number)
        if marker == 0xEB && seg_len >= 10 {
            let hdr = pos + 2; // past Lp
            let ci = u16::from_be_bytes([data[hdr], data[hdr + 1]]);

            if ci == 0x4A50 {
                // "JP" — JUMBF identifier
                let en = u16::from_be_bytes([data[hdr + 2], data[hdr + 3]]) as u32;
                let z = u32::from_be_bytes([
                    data[hdr + 4],
                    data[hdr + 5],
                    data[hdr + 6],
                    data[hdr + 7],
                ]);
                let payload = data[hdr + 8..pos + seg_len].to_vec();
                segments.push((en, z, payload));
            }
        }

        pos += seg_len;
    }

    if segments.is_empty() {
        return None;
    }

    // Group by En, sort by Z within each group, concatenate
    let mut groups: BTreeMap<u32, Vec<(u32, Vec<u8>)>> = BTreeMap::new();
    for (en, z, payload) in segments {
        groups.entry(en).or_default().push((z, payload));
    }

    let mut all_jumbf = Vec::new();
    for (_, mut segs) in groups {
        segs.sort_by_key(|(z, _)| *z);
        for (_, payload) in segs {
            all_jumbf.extend_from_slice(&payload);
        }
    }

    if all_jumbf.is_empty() {
        None
    } else {
        Some(all_jumbf)
    }
}

/// A parsed ISO BMFF / JUMBF box reference.
struct JumbfBox<'a> {
    /// Box type (4 ASCII bytes, e.g. b"jumb", b"jumd", b"cbor")
    box_type: [u8; 4],
    /// Payload bytes (everything after the box header)
    payload: &'a [u8],
}

/// Parse a contiguous sequence of ISO BMFF boxes.
fn parse_boxes(data: &[u8]) -> Vec<JumbfBox<'_>> {
    let mut boxes = Vec::new();
    let mut offset = 0;

    while offset + 8 <= data.len() {
        let size = u32::from_be_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]) as usize;

        let box_type = [
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ];

        let (header_size, box_size) = if size == 0 {
            // Box extends to end of data
            (8, data.len() - offset)
        } else if size == 1 {
            // 64-bit extended size
            if offset + 16 > data.len() {
                break;
            }
            let ext = u64::from_be_bytes([
                data[offset + 8],
                data[offset + 9],
                data[offset + 10],
                data[offset + 11],
                data[offset + 12],
                data[offset + 13],
                data[offset + 14],
                data[offset + 15],
            ]) as usize;
            (16, ext)
        } else {
            (8, size)
        };

        if box_size < header_size || offset + box_size > data.len() {
            break;
        }

        let payload = &data[offset + header_size..offset + box_size];
        boxes.push(JumbfBox { box_type, payload });
        offset += box_size;
    }

    boxes
}

/// UUID for the C2PA signature superbox ("c2cs").
const C2PA_SIGNATURE_UUID: [u8; 16] = [
    0x63, 0x32, 0x63, 0x73, // "c2cs"
    0x00, 0x11, 0x00, 0x10, 0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71,
];

/// Walk the JUMBF box tree and return the CBOR payload of the `c2pa.signature` box.
fn find_signature_cbor_in_jumbf(jumbf_data: &[u8]) -> Option<Vec<u8>> {
    let boxes: Vec<JumbfBox<'_>> = parse_boxes(jumbf_data);
    find_signature_cbor_recursive(&boxes)
}

fn find_signature_cbor_recursive(boxes: &[JumbfBox<'_>]) -> Option<Vec<u8>> {
    for b in boxes {
        if &b.box_type == b"jumb" {
            let children = parse_boxes(b.payload);

            if is_signature_superbox(&children) {
                // Return the first CBOR content box inside the signature superbox
                for child in &children {
                    if &child.box_type == b"cbor" {
                        return Some(child.payload.to_vec());
                    }
                }
                // Fallback: first non-description child that starts with COSE tag 18
                for child in &children {
                    if &child.box_type != b"jumd"
                        && !child.payload.is_empty()
                        && child.payload[0] == 0xD2
                    {
                        return Some(child.payload.to_vec());
                    }
                }
            }

            // Recurse into nested superboxes
            if let Some(result) = find_signature_cbor_recursive(&children) {
                return Some(result);
            }
        }
    }
    None
}

/// Check whether a superbox's children indicate it is the `c2pa.signature` box.
fn is_signature_superbox(children: &[JumbfBox<'_>]) -> bool {
    let desc = match children.first() {
        Some(b) if &b.box_type == b"jumd" => b,
        _ => return false,
    };

    if desc.payload.len() < 16 {
        return false;
    }

    // Primary: UUID match
    if desc.payload[..16] == C2PA_SIGNATURE_UUID {
        return true;
    }

    // Fallback: search for the label string "c2pa.signature" in the payload
    let needle = b"c2pa.signature";
    desc.payload
        .windows(needle.len())
        .any(|w| w == needle.as_slice())
}

/// Parse the ECDSA (r, s) signature from COSE_Sign1 tagged CBOR bytes.
///
/// The COSE_Sign1 signature field stores ECDSA signatures in IEEE P1363
/// format: `r || s` (no DER envelope). For ES256 this is 64 bytes.
fn parse_ecdsa_from_cose_sign1(cose_cbor: &[u8]) -> Option<EcdsaSignature> {
    use coset::{CborSerializable, TaggedCborSerializable};

    // Try tagged CBOR first (standard), then untagged as fallback
    let sign1 = coset::CoseSign1::from_tagged_slice(cose_cbor)
        .or_else(|_| coset::CoseSign1::from_slice(cose_cbor))
        .ok()?;

    let sig = &sign1.signature;

    match sig.len() {
        64 => Some(EcdsaSignature {
            r: sig[..32].to_vec(),
            s: sig[32..].to_vec(),
        }),
        96 => Some(EcdsaSignature {
            r: sig[..48].to_vec(),
            s: sig[48..].to_vec(),
        }),
        _ => None,
    }
}

/// Extract assertions/claims from a C2PA [`c2pa::Manifest`].
fn extract_assertions(manifest: &c2pa::Manifest) -> Vec<ClaimAssertion> {
    manifest
        .assertions()
        .iter()
        .map(|a| ClaimAssertion {
            label: a.label().to_string(),
            data: a.value().cloned().unwrap_or(serde_json::Value::Null),
        })
        .collect()
}

/// Extract ingredient references from a C2PA [`c2pa::Manifest`].
fn extract_ingredients(manifest: &c2pa::Manifest) -> Vec<Ingredient> {
    manifest
        .ingredients()
        .iter()
        .map(|i| Ingredient {
            title: Some(i.title().to_string()),
            format: Some(i.format().to_string()),
            relationship: Some(format!("{:?}", i.relationship())),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_image_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("imgs/test_img.JPG")
    }

    #[test]
    fn test_parse_nonexistent_file() {
        let result = parse_c2pa(Path::new("/nonexistent/file.jpg"));
        // Should be an error (IO) or None (if c2pa treats it as missing metadata)
        assert!(result.is_err() || result.unwrap().is_none());
    }

    #[test]
    fn test_parse_test_image() {
        let path = test_image_path();
        if !path.exists() {
            eprintln!("Skipping: test image not found at {:?}", path);
            return;
        }

        let result = parse_c2pa(&path);
        // Should either succeed with metadata or succeed with None
        match result {
            Ok(Some(metadata)) => {
                assert!(!metadata.active_manifest.is_empty());
                assert!(!metadata.claim_generator.is_empty());
                assert!(!metadata.format.is_empty());
                // Raw manifest store should be populated
                assert!(!metadata.raw_manifest_store.is_null());
            }
            Ok(None) => {
                eprintln!("Test image has no C2PA metadata");
            }
            Err(e) => {
                panic!("Failed to parse test image: {}", e);
            }
        }
    }

    #[test]
    fn test_parse_plain_jpeg() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("plain.jpg");

        // Minimal JPEG without C2PA
        let jpeg: Vec<u8> = vec![
            0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00,
            0x01, 0x01, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0xFF, 0xD9,
        ];
        std::fs::write(&path, &jpeg).unwrap();

        let result = parse_c2pa(&path).unwrap();
        assert!(result.is_none(), "Plain JPEG should have no C2PA metadata");
    }

    #[test]
    fn test_parse_signed_image() {
        // Sign the test image using c2pa crate (same version as our parser)
        // to avoid CBOR format version mismatches with c2patool
        let src_path = test_image_path();
        if !src_path.exists() {
            eprintln!("Skipping: test image not found");
            return;
        }

        let cert_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("certs");
        let cert_path = cert_dir.join("ec_cert.pem");
        let key_path = cert_dir.join("ec_key.pem");
        if !cert_path.exists() || !key_path.exists() {
            eprintln!("Skipping: test certs not found in certs/");
            return;
        }

        let dir = tempfile::tempdir().unwrap();
        let dest_path = dir.path().join("signed.JPG");

        // Use c2pa crate Builder to sign
        let manifest_json = r#"{
            "claim_generator": "BrevisVera/TestSigner/1.0",
            "assertions": [{
                "label": "c2pa.actions",
                "data": {"actions": [{"action": "c2pa.created"}]}
            }]
        }"#;
        let signer = c2pa::create_signer::from_files(
            &cert_path,
            &key_path,
            c2pa::SigningAlg::Es256,
            None,
        )
        .expect("create signer from test certs");

        let mut builder = c2pa::Builder::from_json(manifest_json).unwrap();
        builder
            .sign_file(&*signer, &src_path, &dest_path)
            .expect("sign file");

        // Now parse the signed image with our parser
        let metadata = parse_c2pa(&dest_path)
            .unwrap()
            .expect("signed image should have C2PA");
        let sig_info = metadata
            .signature_info
            .as_ref()
            .expect("should have signature_info");

        // Algorithm should be es256 (ECDSA P-256)
        assert_eq!(sig_info.alg.as_deref(), Some("es256"));

        // ECDSA signature should now be extracted from COSE_Sign1
        let ecdsa_sig = sig_info
            .ecdsa_signature
            .as_ref()
            .expect("should have ECDSA signature extracted from COSE_Sign1");
        assert_eq!(ecdsa_sig.r.len(), 32, "r component should be 32 bytes (P-256)");
        assert_eq!(ecdsa_sig.s.len(), 32, "s component should be 32 bytes (P-256)");
        let (r_hex, s_hex) = ecdsa_sig.to_hex();
        assert!(!r_hex.is_empty());
        assert!(!s_hex.is_empty());
        eprintln!("ECDSA signature extracted:");
        eprintln!("  r: {}", r_hex);
        eprintln!("  s: {}", s_hex);

        // Public key should also be available
        let pub_key = sig_info
            .public_key
            .as_ref()
            .expect("should have public key");
        assert!(pub_key.starts_with("04"), "uncompressed EC point prefix");
        eprintln!("  public_key: {}", pub_key);

        // Should have a claim generator
        assert!(!metadata.claim_generator.is_empty());
    }
}
