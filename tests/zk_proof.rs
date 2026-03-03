//! Integration tests for the ZK Proof Generation Layer.
//!
//! Tests the full pipeline: provenance → editing → ZK proof generation.

use brevis_vera::editor::operations;
use brevis_vera::editor::types::CropParams;
use brevis_vera::provenance::ProvenanceProcessor;
use brevis_vera::zk::prover::ZkProver;
use brevis_vera::zk::types::{
    C2paProofInput, CombinedProofInput, EditingProofInput, EditingRecordInput,
};
use brevis_vera::zk::{SimulatedProver, ZkProof};

use brevis_vera::editor::types::EditOperation;
use std::path::Path;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn prover() -> SimulatedProver {
    SimulatedProver::new()
}

/// Create a small gradient test image as PNG bytes.
fn make_test_image(width: u32, height: u32) -> Vec<u8> {
    let mut img = image::RgbaImage::new(width, height);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgba([x as u8, y as u8, ((x + y) % 256) as u8, 255]);
    }
    let mut buf = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut buf);
    image::ImageEncoder::write_image(
        encoder,
        img.as_raw(),
        width,
        height,
        image::ExtendedColorType::Rgba8,
    )
    .unwrap();
    buf
}

fn sha256_hex(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    hex::encode(Sha256::digest(data))
}

// ---------------------------------------------------------------------------
// C2PA proof tests with real image
// ---------------------------------------------------------------------------

#[test]
fn test_c2pa_proof_with_test_image() {
    let img_path = Path::new("imgs/test_img.JPG");
    if !img_path.exists() {
        eprintln!("Skipping: test image not found");
        return;
    }

    let processor = ProvenanceProcessor::new();
    let result = processor.process(img_path).unwrap();

    // The test image has C2PA metadata
    let zkvm_input = result.zkvm_input.expect("Test image should have C2PA data");

    let input = C2paProofInput {
        c2pa_data: zkvm_input.c2pa_data,
        image_hash: zkvm_input.image_hash.to_hex(),
    };

    let p = prover();
    let proof = p.prove_c2pa(&input).unwrap();

    assert!(proof.public_inputs.c2pa_verified);
    assert!(!proof.public_inputs.editing_verified);
    assert!(!proof.proof_bytes.is_empty());
    assert!(p.verify(&proof).unwrap());
}

// ---------------------------------------------------------------------------
// Editing proof tests
// ---------------------------------------------------------------------------

#[test]
fn test_editing_proof_with_real_crop() {
    // Create a test image, crop it, generate proof
    let original = make_test_image(200, 200);
    let orig_hash = sha256_hex(&original);

    let crop_result = operations::crop(
        &original,
        &CropParams {
            x: 10,
            y: 10,
            width: 100,
            height: 100,
        },
    )
    .unwrap();

    let edited_hash = sha256_hex(&crop_result.image_bytes);

    let input = EditingProofInput {
        original_image_hash: orig_hash.clone(),
        edited_image_hash: edited_hash.clone(),
        editing_records: vec![EditingRecordInput {
            operation: EditOperation::Crop,
            parameters: serde_json::json!({
                "x": 10, "y": 10, "width": 100, "height": 100
            }),
            input_hash: orig_hash,
            output_hash: edited_hash,
            raw_pixels: None,
            pixel_width: None,
            pixel_height: None,
        }],
    };

    let p = prover();
    let proof = p.prove_editing(&input).unwrap();

    assert!(proof.public_inputs.editing_verified);
    assert_eq!(proof.public_inputs.operations_applied, vec![EditOperation::Crop]);
    assert!(p.verify(&proof).unwrap());
}

#[test]
fn test_editing_proof_chain_crop_then_resize() {
    let original = make_test_image(300, 300);
    let orig_hash = sha256_hex(&original);

    // Step 1: Crop
    let crop_result = operations::crop(
        &original,
        &CropParams {
            x: 0,
            y: 0,
            width: 200,
            height: 200,
        },
    )
    .unwrap();
    let cropped_hash = sha256_hex(&crop_result.image_bytes);

    // Step 2: Resize
    let resize_result = operations::resize(
        &crop_result.image_bytes,
        &brevis_vera::editor::types::ResizeParams {
            width: 100,
            height: 100,
        },
    )
    .unwrap();
    let resized_hash = sha256_hex(&resize_result.image_bytes);

    let input = EditingProofInput {
        original_image_hash: orig_hash.clone(),
        edited_image_hash: resized_hash.clone(),
        editing_records: vec![
            EditingRecordInput {
                operation: EditOperation::Crop,
                parameters: serde_json::json!({
                    "x": 0, "y": 0, "width": 200, "height": 200
                }),
                input_hash: orig_hash,
                output_hash: cropped_hash.clone(),
                raw_pixels: None,
                pixel_width: None,
                pixel_height: None,
            },
            EditingRecordInput {
                operation: EditOperation::Resize,
                parameters: serde_json::json!({"width": 100, "height": 100}),
                input_hash: cropped_hash,
                output_hash: resized_hash,
                raw_pixels: None,
                pixel_width: None,
                pixel_height: None,
            },
        ],
    };

    let p = prover();
    let proof = p.prove_editing(&input).unwrap();

    assert!(proof.public_inputs.editing_verified);
    assert_eq!(proof.public_inputs.operations_applied.len(), 2);
    assert!(p.verify(&proof).unwrap());
}

// ---------------------------------------------------------------------------
// Combined proof tests
// ---------------------------------------------------------------------------

#[test]
fn test_combined_proof_with_test_image() {
    let img_path = Path::new("imgs/test_img.JPG");
    if !img_path.exists() {
        eprintln!("Skipping: test image not found");
        return;
    }

    let processor = ProvenanceProcessor::new();
    let result = processor.process(img_path).unwrap();

    let zkvm_input = result.zkvm_input.expect("Should have C2PA data");
    let orig_hash = zkvm_input.image_hash.to_hex();

    // Crop the original image
    let crop_result = operations::crop(
        &result.original_image,
        &CropParams {
            x: 100,
            y: 100,
            width: 500,
            height: 500,
        },
    )
    .unwrap();
    let edited_hash = sha256_hex(&crop_result.image_bytes);

    let input = CombinedProofInput {
        c2pa_data: Some(zkvm_input.c2pa_data),
        original_image_hash: orig_hash.clone(),
        editing_records: vec![EditingRecordInput {
            operation: EditOperation::Crop,
            parameters: serde_json::json!({
                "x": 100, "y": 100, "width": 500, "height": 500
            }),
            input_hash: orig_hash,
            output_hash: edited_hash.clone(),
            raw_pixels: None,
            pixel_width: None,
            pixel_height: None,
        }],
        edited_image_hash: Some(edited_hash),
    };

    let p = prover();
    let proof = p.prove_combined(&input).unwrap();

    assert!(proof.public_inputs.c2pa_verified);
    assert!(proof.public_inputs.editing_verified);
    assert_eq!(proof.public_inputs.operations_applied, vec![EditOperation::Crop]);
    assert!(p.verify(&proof).unwrap());
}

#[test]
fn test_combined_proof_no_edits() {
    let img_path = Path::new("imgs/test_img.JPG");
    if !img_path.exists() {
        eprintln!("Skipping: test image not found");
        return;
    }

    let processor = ProvenanceProcessor::new();
    let result = processor.process(img_path).unwrap();
    let zkvm_input = result.zkvm_input.expect("Should have C2PA data");

    let input = CombinedProofInput {
        c2pa_data: Some(zkvm_input.c2pa_data),
        original_image_hash: zkvm_input.image_hash.to_hex(),
        editing_records: vec![],
        edited_image_hash: None,
    };

    let p = prover();
    let proof = p.prove_combined(&input).unwrap();

    assert!(proof.public_inputs.c2pa_verified);
    assert!(!proof.public_inputs.editing_verified);
    assert!(p.verify(&proof).unwrap());
}

// ---------------------------------------------------------------------------
// Proof serialization tests
// ---------------------------------------------------------------------------

#[test]
fn test_proof_json_roundtrip_integration() {
    let original = make_test_image(100, 100);
    let orig_hash = sha256_hex(&original);

    let crop_result = operations::crop(
        &original,
        &CropParams {
            x: 0,
            y: 0,
            width: 50,
            height: 50,
        },
    )
    .unwrap();
    let edited_hash = sha256_hex(&crop_result.image_bytes);

    let input = EditingProofInput {
        original_image_hash: orig_hash.clone(),
        edited_image_hash: edited_hash.clone(),
        editing_records: vec![EditingRecordInput {
            operation: EditOperation::Crop,
            parameters: serde_json::json!({"x": 0, "y": 0, "width": 50, "height": 50}),
            input_hash: orig_hash,
            output_hash: edited_hash,
            raw_pixels: None,
            pixel_width: None,
            pixel_height: None,
        }],
    };

    let p = prover();
    let proof = p.prove_editing(&input).unwrap();

    // JSON roundtrip
    let json = proof.to_json().unwrap();
    let restored = ZkProof::from_json(&json).unwrap();
    assert!(p.verify(&restored).unwrap());

    // Binary roundtrip
    let bytes = proof.to_bytes().unwrap();
    let restored_binary = ZkProof::from_bytes(&bytes).unwrap();
    assert!(p.verify(&restored_binary).unwrap());
}

// ---------------------------------------------------------------------------
// Benchmark test
// ---------------------------------------------------------------------------

#[test]
fn test_proof_generation_benchmark() {
    let p = prover();

    // Generate multiple proofs and check timing
    let mut total_ms = 0u64;
    let iterations = 10;

    for _ in 0..iterations {
        let input = CombinedProofInput {
            c2pa_data: Some(brevis_vera::provenance::types::C2paVerificationData {
                signature_info: Some(brevis_vera::provenance::types::SignatureInfo {
                    issuer: Some("CN=Test".to_string()),
                    time: Some("2026-01-01T00:00:00Z".to_string()),
                    cert_serial_number: Some("123".to_string()),
                    alg: Some("Es256".to_string()),
                    ecdsa_signature: None,
                    public_key: None,
                }),
                assertions: vec![],
                active_manifest: "urn:c2pa:test".to_string(),
                claim_generator: "Test/1.0".to_string(),
                ecdsa_signature: None,
                public_key: None,
            }),
            original_image_hash: "abcdef1234567890".to_string(),
            editing_records: vec![
                EditingRecordInput {
                    operation: EditOperation::Crop,
                    parameters: serde_json::json!({"x": 0, "y": 0, "width": 100, "height": 100}),
                    input_hash: "abcdef1234567890".to_string(),
                    output_hash: "crop_output_hash".to_string(),
                    raw_pixels: None,
                    pixel_width: None,
                    pixel_height: None,
                },
                EditingRecordInput {
                    operation: EditOperation::Resize,
                    parameters: serde_json::json!({"width": 50, "height": 50}),
                    input_hash: "crop_output_hash".to_string(),
                    output_hash: "final_hash".to_string(),
                    raw_pixels: None,
                    pixel_width: None,
                    pixel_height: None,
                },
            ],
            edited_image_hash: Some("final_hash".to_string()),
        };

        let proof = p.prove_combined(&input).unwrap();
        total_ms += proof.metadata.generation_time_ms;
    }

    let avg_ms = total_ms / iterations;
    println!("Average proof generation time (simulated): {avg_ms}ms over {iterations} iterations");
    // Simulated prover should be very fast
    assert!(avg_ms < 100, "Simulated prover too slow: {avg_ms}ms avg");
}

// ---------------------------------------------------------------------------
// Tamper detection test
// ---------------------------------------------------------------------------

#[test]
fn test_tamper_detection() {
    let p = prover();

    let original = make_test_image(100, 100);
    let orig_hash = sha256_hex(&original);

    let crop_result = operations::crop(
        &original,
        &CropParams {
            x: 10,
            y: 10,
            width: 50,
            height: 50,
        },
    )
    .unwrap();
    let edited_hash = sha256_hex(&crop_result.image_bytes);

    let input = EditingProofInput {
        original_image_hash: orig_hash.clone(),
        edited_image_hash: edited_hash.clone(),
        editing_records: vec![EditingRecordInput {
            operation: EditOperation::Crop,
            parameters: serde_json::json!({"x": 10, "y": 10, "width": 50, "height": 50}),
            input_hash: orig_hash,
            output_hash: edited_hash,
            raw_pixels: None,
            pixel_width: None,
            pixel_height: None,
        }],
    };

    let proof = p.prove_editing(&input).unwrap();
    assert!(p.verify(&proof).unwrap());

    // Tamper with proof bytes
    let mut tampered = proof.clone();
    tampered.proof_bytes[0] ^= 0xFF;
    assert!(!p.verify(&tampered).unwrap());

    // Tamper with public inputs
    let mut tampered2 = proof.clone();
    tampered2.public_inputs.original_image_hash = "tampered_hash".to_string();
    assert!(!p.verify(&tampered2).unwrap());

    // Tamper with operations list
    let mut tampered3 = proof;
    tampered3.public_inputs.operations_applied.push(EditOperation::Resize);
    assert!(!p.verify(&tampered3).unwrap());
}
