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

    let signature_info = extract_signature_info(active_manifest_obj);
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
fn extract_signature_info(manifest: &c2pa::Manifest) -> Option<SignatureInfo> {
    let sig = manifest.signature_info()?;

    // Try to extract ECDSA signature and public key from the manifest
    let (ecdsa_signature, public_key) = extract_ecdsa_data(manifest);

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
/// Attempts to extract raw signature bytes from the manifest store for
/// ZKVM-based cryptographic verification.
fn extract_ecdsa_data(manifest: &c2pa::Manifest) -> (Option<EcdsaSignature>, Option<String>) {
    // The c2pa crate doesn't directly expose raw signature bytes through its public API.
    // The signature is embedded in JUMBF boxes in COSE format.
    //
    // We can try to extract the public key from the certificate chain.

    // Get signature info which contains certificate chain
    let sig_info = match manifest.signature_info() {
        Some(info) => info,
        None => return (None, None),
    };

    // Extract public key from the first certificate in the chain
    let cert_chain = sig_info.cert_chain();
    if cert_chain.is_empty() {
        return (None, None);
    }

    // Parse the PEM certificate chain to extract public key
    // The certificate chain is in PEM format
    let public_key = extract_public_key_from_pem(cert_chain);

    // Note: The actual signature r, s bytes are not available through the public API.
    // For ZKVM verification, we would need to either:
    // 1. Parse the JUMBF binary boxes directly
    // 2. Use a configuration-based approach for known test keys
    // 3. Implement COSE signature parsing

    (None, public_key)
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

        // Should have a claim generator
        assert!(!metadata.claim_generator.is_empty());
    }
}
