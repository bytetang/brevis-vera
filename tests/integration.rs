//! Integration tests for the Capture & Provenance Layer.
//!
//! These tests exercise the full provenance pipeline with real and
//! synthetic media files.

use brevis_vera::provenance::{MediaFormat, ProvenanceProcessor};
use std::path::{Path, PathBuf};

fn test_image_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("imgs/test_img.JPG")
}

// -----------------------------------------------------------------------
// Test with real C2PA-signed image (Sony ILCE-1)
// -----------------------------------------------------------------------

#[test]
fn test_process_signed_jpeg() {
    let path = test_image_path();
    if !path.exists() {
        eprintln!("Skipping: test image not at {:?}", path);
        return;
    }

    let processor = ProvenanceProcessor::new();
    let result = processor.process(&path).unwrap();

    // Format detection
    assert_eq!(result.format, MediaFormat::Jpeg);

    // Image data
    assert!(!result.original_image.is_empty());
    assert_ne!(result.image_hash.0, [0u8; 32]);

    // Print metadata for inspection
    if let Some(ref metadata) = result.c2pa_metadata {
        println!("=== C2PA Metadata ===");
        println!("Active manifest : {}", metadata.active_manifest);
        println!("Claim generator : {}", metadata.claim_generator);
        println!("Title           : {:?}", metadata.title);
        println!("Format          : {}", metadata.format);

        if let Some(ref sig) = metadata.signature_info {
            println!("--- Signature ---");
            println!("  Issuer       : {:?}", sig.issuer);
            println!("  Time         : {:?}", sig.time);
            println!("  Serial       : {:?}", sig.cert_serial_number);
            println!("  Algorithm    : {:?}", sig.alg);
        }

        println!("--- Assertions ({}) ---", metadata.assertions.len());
        for a in &metadata.assertions {
            println!("  - {}", a.label);
        }

        println!("--- Ingredients ({}) ---", metadata.ingredients.len());
        for i in &metadata.ingredients {
            println!("  - {:?} ({:?})", i.title, i.format);
        }

        // ZKVM input should be built
        assert!(result.zkvm_input.is_some());
        let zkvm = result.zkvm_input.as_ref().unwrap();
        println!("--- ZKVM Input ---");
        println!("  Image hash: {}", zkvm.image_hash);

        // Verify ZKVM input consistency
        assert_eq!(zkvm.image_hash, result.image_hash);
        assert_eq!(
            zkvm.c2pa_data.active_manifest,
            metadata.active_manifest
        );
        assert_eq!(
            zkvm.c2pa_data.claim_generator,
            metadata.claim_generator
        );
    } else {
        println!("No C2PA metadata found in test image (may not be C2PA-signed)");
    }
}

#[test]
fn test_image_hash_stability() {
    let path = test_image_path();
    if !path.exists() {
        return;
    }

    let processor = ProvenanceProcessor::new();
    let r1 = processor.process(&path).unwrap();
    let r2 = processor.process(&path).unwrap();

    assert_eq!(r1.image_hash, r2.image_hash);
    assert_eq!(r1.original_image.len(), r2.original_image.len());
}

// -----------------------------------------------------------------------
// Test with file that has no C2PA metadata
// -----------------------------------------------------------------------

#[test]
fn test_process_jpeg_without_c2pa() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("plain.jpg");

    // Minimal valid JPEG: SOI + APP0 (JFIF) + EOI
    let minimal_jpeg: Vec<u8> = vec![
        0xFF, 0xD8, // SOI
        0xFF, 0xE0, // APP0 marker
        0x00, 0x10, // length = 16
        0x4A, 0x46, 0x49, 0x46, 0x00, // JFIF\0
        0x01, 0x01, // version 1.1
        0x00, // aspect ratio
        0x00, 0x01, // X density
        0x00, 0x01, // Y density
        0x00, 0x00, // thumbnail
        0xFF, 0xD9, // EOI
    ];
    std::fs::write(&path, &minimal_jpeg).unwrap();

    let processor = ProvenanceProcessor::new();
    let result = processor.process(&path).unwrap();

    assert_eq!(result.format, MediaFormat::Jpeg);
    assert!(result.c2pa_metadata.is_none(), "Plain JPEG should have no C2PA metadata");
    assert!(result.zkvm_input.is_none(), "No ZKVM input without C2PA metadata");
    assert!(!result.original_image.is_empty());
}

// -----------------------------------------------------------------------
// Edge cases
// -----------------------------------------------------------------------

#[test]
fn test_unsupported_format() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.gif");
    std::fs::write(&path, b"GIF89a\x01\x00\x01\x00").unwrap();

    let processor = ProvenanceProcessor::new();
    let result = processor.process(&path);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        err.to_string().contains("Unsupported"),
        "Error should mention unsupported format: {}",
        err
    );
}

#[test]
fn test_missing_file() {
    let processor = ProvenanceProcessor::new();
    let result = processor.process(Path::new("/nonexistent/path/image.jpg"));

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        err.to_string().contains("IO"),
        "Error should be IO: {}",
        err
    );
}

#[test]
fn test_empty_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("empty.jpg");
    std::fs::write(&path, b"").unwrap();

    let processor = ProvenanceProcessor::new();
    let result = processor.process(&path);

    assert!(result.is_err());
}

#[test]
fn test_truncated_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("truncated.jpg");
    // Only JPEG SOI marker, no content
    std::fs::write(&path, &[0xFF, 0xD8]).unwrap();

    let processor = ProvenanceProcessor::new();
    let result = processor.process(&path);

    assert!(result.is_err());
}

// -----------------------------------------------------------------------
// ZKVM input serialization
// -----------------------------------------------------------------------

#[test]
fn test_zkvm_input_serializable() {
    let path = test_image_path();
    if !path.exists() {
        return;
    }

    let processor = ProvenanceProcessor::new();
    let result = processor.process(&path).unwrap();

    if let Some(ref zkvm_input) = result.zkvm_input {
        // Should serialize to JSON without errors
        let json = serde_json::to_string_pretty(zkvm_input).unwrap();
        assert!(!json.is_empty());
        println!("ZKVM input JSON length: {} bytes", json.len());

        // Should deserialize back
        let deserialized: brevis_vera::provenance::ZkvmInput =
            serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.image_hash, zkvm_input.image_hash);
    }
}

#[test]
fn test_provenance_result_serializable() {
    let path = test_image_path();
    if !path.exists() {
        return;
    }

    let processor = ProvenanceProcessor::new();
    let result = processor.process(&path).unwrap();

    // ProvenanceResult should serialize (original_image is skipped)
    let json = serde_json::to_string_pretty(&result).unwrap();
    assert!(!json.is_empty());
    // original_image is #[serde(skip)] so should not appear
    assert!(!json.contains("original_image"));
}
