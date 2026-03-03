//! Original image extraction and ZKVM input preparation.
//!
//! This module handles:
//! - Computing SHA-256 hashes of image data
//! - Building the [`ZkvmInput`] structure for the ZK Proof Layer
//!
//! # ZKVM Integration
//!
//! The [`ZkvmInput`] struct produced by [`build_zkvm_input`] contains all data
//! needed for the ZKVM circuit to perform privacy-preserving C2PA verification:
//!
//! 1. Signature info (ECDSA P-256 parameters)
//! 2. Claims/assertions from the C2PA manifest
//! 3. SHA-256 hash of the original image
//!
//! The actual ECDSA verification happens inside the ZKVM circuit
//! (see ZK Proof Layer).

use sha2::{Digest, Sha256};

use super::types::{C2paMetadata, C2paVerificationData, ImageHash, ZkvmInput};

/// Compute the SHA-256 hash of image bytes.
///
/// This hash is used as part of the ZKVM input to bind the image content
/// to the C2PA manifest during in-circuit verification.
///
/// # Arguments
/// * `data` - Raw image file bytes
///
/// # Returns
/// An [`ImageHash`] containing the 32-byte SHA-256 digest.
pub fn compute_image_hash(data: &[u8]) -> ImageHash {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    ImageHash(hash)
}

/// Build a ZKVM input from C2PA metadata and an image hash.
///
/// Creates the [`ZkvmInput`] struct that will be passed to the
/// ZK Proof Layer for privacy-preserving C2PA verification inside
/// the ZKVM circuit.
///
/// # Arguments
/// * `metadata` - Extracted C2PA metadata from [`crate::provenance::parser::parse_c2pa`]
/// * `image_hash` - SHA-256 hash from [`compute_image_hash`]
///
/// # Returns
/// A [`ZkvmInput`] ready for ZKVM processing.
pub fn build_zkvm_input(metadata: &C2paMetadata, image_hash: ImageHash) -> ZkvmInput {
    // Extract ECDSA signature and public key from signature_info if available
    let ecdsa_signature = metadata.signature_info.as_ref()
        .and_then(|sig| sig.ecdsa_signature.clone());
    let public_key = metadata.signature_info.as_ref()
        .and_then(|sig| sig.public_key.clone());

    ZkvmInput {
        c2pa_data: C2paVerificationData {
            signature_info: metadata.signature_info.clone(),
            assertions: metadata.assertions.clone(),
            active_manifest: metadata.active_manifest.clone(),
            claim_generator: metadata.claim_generator.clone(),
            ecdsa_signature,
            public_key,
        },
        image_hash,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provenance::types::{ClaimAssertion, SignatureInfo};

    #[test]
    fn test_compute_image_hash_deterministic() {
        let data = b"test image data for hashing";
        let hash1 = compute_image_hash(data);
        let hash2 = compute_image_hash(data);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_compute_image_hash_different_inputs() {
        let hash1 = compute_image_hash(b"image A");
        let hash2 = compute_image_hash(b"image B");
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_compute_image_hash_empty() {
        let hash = compute_image_hash(b"");
        // SHA-256 of empty input is a well-known constant
        assert_eq!(
            hash.to_hex(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_compute_image_hash_known_value() {
        // SHA-256("hello") = 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
        let hash = compute_image_hash(b"hello");
        assert_eq!(
            hash.to_hex(),
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_image_hash_display() {
        let hash = ImageHash([0xAB; 32]);
        let display = format!("{}", hash);
        assert_eq!(display.len(), 64); // 32 bytes = 64 hex chars
        assert!(display.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_build_zkvm_input_full() {
        let metadata = C2paMetadata {
            active_manifest: "urn:uuid:12345678-1234-1234-1234-123456789abc".to_string(),
            claim_generator: "Sony ILCE-1 / 1.0".to_string(),
            title: Some("test_img.JPG".to_string()),
            format: "image/jpeg".to_string(),
            signature_info: Some(SignatureInfo {
                issuer: Some("CN=Sony Corporation".to_string()),
                time: Some("2024-06-15T08:30:00Z".to_string()),
                cert_serial_number: Some("ABCDEF1234567890".to_string()),
                alg: Some("Es256".to_string()),
                ecdsa_signature: None,
                public_key: None,
            }),
            assertions: vec![
                ClaimAssertion {
                    label: "c2pa.actions".to_string(),
                    data: serde_json::json!({"actions": [{"action": "c2pa.created"}]}),
                },
                ClaimAssertion {
                    label: "c2pa.hash.data".to_string(),
                    data: serde_json::json!({"name": "jumbf manifest"}),
                },
            ],
            ingredients: vec![],
            raw_manifest_store: serde_json::Value::Null,
        };

        let hash = ImageHash([0x42; 32]);
        let input = build_zkvm_input(&metadata, hash);

        assert_eq!(input.image_hash, hash);
        assert_eq!(
            input.c2pa_data.active_manifest,
            "urn:uuid:12345678-1234-1234-1234-123456789abc"
        );
        assert_eq!(input.c2pa_data.claim_generator, "Sony ILCE-1 / 1.0");
        assert!(input.c2pa_data.signature_info.is_some());
        assert_eq!(
            input.c2pa_data.signature_info.as_ref().unwrap().alg.as_deref(),
            Some("Es256")
        );
        assert_eq!(input.c2pa_data.assertions.len(), 2);
    }

    #[test]
    fn test_build_zkvm_input_no_signature() {
        let metadata = C2paMetadata {
            active_manifest: "urn:uuid:test".to_string(),
            claim_generator: "Unknown".to_string(),
            title: None,
            format: "image/jpeg".to_string(),
            signature_info: None,
            assertions: vec![],
            ingredients: vec![],
            raw_manifest_store: serde_json::Value::Null,
        };

        let hash = compute_image_hash(b"test data");
        let input = build_zkvm_input(&metadata, hash);

        assert!(input.c2pa_data.signature_info.is_none());
        assert!(input.c2pa_data.assertions.is_empty());
        assert_eq!(input.image_hash, hash);
    }

    #[test]
    fn test_zkvm_input_json_roundtrip() {
        let metadata = C2paMetadata {
            active_manifest: "urn:uuid:test".to_string(),
            claim_generator: "TestGen/1.0".to_string(),
            title: None,
            format: "image/jpeg".to_string(),
            signature_info: Some(SignatureInfo {
                issuer: Some("CN=Test CA".to_string()),
                time: None,
                cert_serial_number: None,
                alg: Some("Es256".to_string()),
                ecdsa_signature: None,
                public_key: None,
            }),
            assertions: vec![ClaimAssertion {
                label: "c2pa.actions".to_string(),
                data: serde_json::json!({"actions": []}),
            }],
            ingredients: vec![],
            raw_manifest_store: serde_json::Value::Null,
        };

        let hash = ImageHash([0x00; 32]);
        let input = build_zkvm_input(&metadata, hash);

        // Serialize to JSON
        let json = serde_json::to_string_pretty(&input).unwrap();
        assert!(json.contains("Es256"));
        assert!(json.contains("image_hash"));
        assert!(json.contains("c2pa_data"));

        // Deserialize back
        let deserialized: ZkvmInput = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.image_hash, hash);
        assert_eq!(deserialized.c2pa_data.claim_generator, "TestGen/1.0");
        assert_eq!(
            deserialized
                .c2pa_data
                .signature_info
                .as_ref()
                .unwrap()
                .alg
                .as_deref(),
            Some("Es256")
        );
    }
}
