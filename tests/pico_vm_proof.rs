//! Integration tests for Pico ZKVM proof generation.
//!
//! These tests build real ZK proofs using the compiled guest ELF binary
//! and the PicoProver. They verify that crop, resize, and rotate operations
//! produce valid proofs with correct public outputs.
//!
//! # Prerequisites
//!
//! Build the guest ELF before running:
//! ```bash
//! cd zk-guest/app && cargo pico build
//! ```
//!
//! Run with the `pico` feature:
//! ```bash
//! cargo test --features pico --test pico_vm_proof
//! ```

#![cfg(feature = "pico")]

use brevis_vera::editor::operations;
use brevis_vera::editor::types::{CropParams, EditOperation, ResizeParams, RotateParams, RotationAngle};
use brevis_vera::zk::prover::{PicoProver, ZkProver};
use brevis_vera::zk::types::{EditingProofInput, EditingRecordInput};

const ELF_PATH: &str = "zk-guest/app/elf/riscv32im-pico-zkvm-elf";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Create a tiny test image as PNG bytes (kept small for fast ZKVM proving).
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

fn pico_prover() -> PicoProver {
    PicoProver::new(ELF_PATH).expect(
        "Failed to load guest ELF. Build it first: cd zk-guest/app && cargo pico build",
    )
}

// ---------------------------------------------------------------------------
// Crop
// ---------------------------------------------------------------------------

#[test]
fn test_pico_crop() {
    let original = make_test_image(16, 16);
    let orig_hash = sha256_hex(&original);

    let crop_result = operations::crop(
        &original,
        &CropParams {
            x: 2,
            y: 2,
            width: 8,
            height: 8,
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
                "x": 2, "y": 2, "width": 8, "height": 8
            }),
            input_hash: orig_hash,
            output_hash: edited_hash,
            raw_pixels: None,
            pixel_width: None,
            pixel_height: None,
        }],
    };

    let prover = pico_prover();
    let proof = prover.prove_editing(&input).unwrap();

    assert!(proof.public_inputs.editing_verified, "editing should be verified");
    assert_eq!(
        proof.public_inputs.operations_applied,
        vec![EditOperation::Crop]
    );
    assert_eq!(proof.metadata.prover_type, "pico");
    println!(
        "Pico crop proof generated in {}ms ({} bytes)",
        proof.metadata.generation_time_ms,
        proof.proof_bytes.len()
    );
}

/// End-to-end Pico proof with crop **re-execution** inside the ZKVM.
///
/// Unlike `test_pico_crop` (parameter-only), this test supplies raw RGBA
/// pixels so the guest program re-executes the crop, hashes the result,
/// and compares against the claimed output hash.
#[test]
fn test_pico_crop_reexecution() {
    let original_png = make_test_image(16, 16);

    // Extract raw RGBA pixels from the PNG (same format the guest will use)
    let (raw_pixels, img_w, img_h) = operations::extract_raw_rgba(&original_png).unwrap();
    assert_eq!(img_w, 16);
    assert_eq!(img_h, 16);

    let crop_params = CropParams {
        x: 2,
        y: 2,
        width: 8,
        height: 8,
    };

    // Run the crop — its record already hashes raw RGBA pixels
    let crop_result = operations::crop(&original_png, &crop_params).unwrap();
    let record = &crop_result.record;

    // The hashes in the record are over raw pixels, which is what the ZKVM
    // guest will recompute. Use them directly.
    let input = EditingProofInput {
        original_image_hash: record.original_image_hash.clone(),
        edited_image_hash: record.edited_image_hash.clone(),
        editing_records: vec![EditingRecordInput {
            operation: EditOperation::Crop,
            parameters: serde_json::json!({
                "x": crop_params.x,
                "y": crop_params.y,
                "width": crop_params.width,
                "height": crop_params.height,
                "source_width": img_w,
                "source_height": img_h,
            }),
            input_hash: record.original_image_hash.clone(),
            output_hash: record.edited_image_hash.clone(),
            raw_pixels: Some(raw_pixels),
            pixel_width: Some(img_w),
            pixel_height: Some(img_h),
        }],
    };

    let prover = pico_prover();
    let proof = prover.prove_editing(&input).unwrap();

    assert!(
        proof.public_inputs.editing_verified,
        "crop re-execution should be verified"
    );
    assert_eq!(
        proof.public_inputs.operations_applied,
        vec![EditOperation::Crop]
    );
    assert_eq!(proof.metadata.prover_type, "pico");
    println!(
        "Pico crop re-execution proof generated in {}ms ({} bytes)",
        proof.metadata.generation_time_ms,
        proof.proof_bytes.len()
    );
}

// ---------------------------------------------------------------------------
// Resize
// ---------------------------------------------------------------------------

#[test]
fn test_pico_resize() {
    let original = make_test_image(16, 16);
    let orig_hash = sha256_hex(&original);

    let resize_result = operations::resize(
        &original,
        &ResizeParams {
            width: 8,
            height: 8,
        },
    )
    .unwrap();
    let edited_hash = sha256_hex(&resize_result.image_bytes);

    let input = EditingProofInput {
        original_image_hash: orig_hash.clone(),
        edited_image_hash: edited_hash.clone(),
        editing_records: vec![EditingRecordInput {
            operation: EditOperation::Resize,
            parameters: serde_json::json!({"width": 8, "height": 8}),
            input_hash: orig_hash,
            output_hash: edited_hash,
            raw_pixels: None,
            pixel_width: None,
            pixel_height: None,
        }],
    };

    let prover = pico_prover();
    let proof = prover.prove_editing(&input).unwrap();

    assert!(proof.public_inputs.editing_verified, "editing should be verified");
    assert_eq!(
        proof.public_inputs.operations_applied,
        vec![EditOperation::Resize]
    );
    assert_eq!(proof.metadata.prover_type, "pico");
    println!(
        "Pico resize proof generated in {}ms ({} bytes)",
        proof.metadata.generation_time_ms,
        proof.proof_bytes.len()
    );
}

/// End-to-end Pico proof with resize **re-execution** inside the ZKVM.
///
/// Supplies raw RGBA pixels so the guest re-executes nearest-neighbor
/// resize and verifies the output hash matches.
#[test]
fn test_pico_resize_reexecution() {
    let original_png = make_test_image(16, 16);

    let (raw_pixels, img_w, img_h) = operations::extract_raw_rgba(&original_png).unwrap();
    assert_eq!(img_w, 16);
    assert_eq!(img_h, 16);

    let resize_params = ResizeParams { width: 8, height: 8 };

    let resize_result = operations::resize(&original_png, &resize_params).unwrap();
    let record = &resize_result.record;

    let input = EditingProofInput {
        original_image_hash: record.original_image_hash.clone(),
        edited_image_hash: record.edited_image_hash.clone(),
        editing_records: vec![EditingRecordInput {
            operation: EditOperation::Resize,
            parameters: serde_json::json!({
                "original_width": img_w,
                "original_height": img_h,
                "new_width": resize_params.width,
                "new_height": resize_params.height,
            }),
            input_hash: record.original_image_hash.clone(),
            output_hash: record.edited_image_hash.clone(),
            raw_pixels: Some(raw_pixels),
            pixel_width: Some(img_w),
            pixel_height: Some(img_h),
        }],
    };

    let prover = pico_prover();
    let proof = prover.prove_editing(&input).unwrap();

    assert!(
        proof.public_inputs.editing_verified,
        "resize re-execution should be verified"
    );
    assert_eq!(
        proof.public_inputs.operations_applied,
        vec![EditOperation::Resize]
    );
    assert_eq!(proof.metadata.prover_type, "pico");
    println!(
        "Pico resize re-execution proof generated in {}ms ({} bytes)",
        proof.metadata.generation_time_ms,
        proof.proof_bytes.len()
    );
}

// ---------------------------------------------------------------------------
// Rotate
// ---------------------------------------------------------------------------

#[test]
fn test_pico_rotate() {
    let original = make_test_image(16, 12);
    let orig_hash = sha256_hex(&original);

    let rotate_result = operations::rotate(
        &original,
        &RotateParams {
            angle: RotationAngle::Deg90,
        },
    )
    .unwrap();
    let edited_hash = sha256_hex(&rotate_result.image_bytes);

    let input = EditingProofInput {
        original_image_hash: orig_hash.clone(),
        edited_image_hash: edited_hash.clone(),
        editing_records: vec![EditingRecordInput {
            operation: EditOperation::Rotate,
            parameters: serde_json::json!({"angle": 90}),
            input_hash: orig_hash,
            output_hash: edited_hash,
            raw_pixels: None,
            pixel_width: None,
            pixel_height: None,
        }],
    };

    let prover = pico_prover();
    let proof = prover.prove_editing(&input).unwrap();

    assert!(proof.public_inputs.editing_verified, "editing should be verified");
    assert_eq!(
        proof.public_inputs.operations_applied,
        vec![EditOperation::Rotate]
    );
    assert_eq!(proof.metadata.prover_type, "pico");
    println!(
        "Pico rotate proof generated in {}ms ({} bytes)",
        proof.metadata.generation_time_ms,
        proof.proof_bytes.len()
    );
}

/// End-to-end Pico proof with rotate **re-execution** inside the ZKVM.
///
/// Supplies raw RGBA pixels so the guest re-executes 90° rotation by
/// pixel permutation and verifies the output hash matches.
#[test]
fn test_pico_rotate_reexecution() {
    let original_png = make_test_image(16, 12);

    let (raw_pixels, img_w, img_h) = operations::extract_raw_rgba(&original_png).unwrap();
    assert_eq!(img_w, 16);
    assert_eq!(img_h, 12);

    let rotate_params = RotateParams { angle: RotationAngle::Deg90 };

    let rotate_result = operations::rotate(&original_png, &rotate_params).unwrap();
    let record = &rotate_result.record;

    let input = EditingProofInput {
        original_image_hash: record.original_image_hash.clone(),
        edited_image_hash: record.edited_image_hash.clone(),
        editing_records: vec![EditingRecordInput {
            operation: EditOperation::Rotate,
            parameters: serde_json::json!({
                "angle": 90,
            }),
            input_hash: record.original_image_hash.clone(),
            output_hash: record.edited_image_hash.clone(),
            raw_pixels: Some(raw_pixels),
            pixel_width: Some(img_w),
            pixel_height: Some(img_h),
        }],
    };

    let prover = pico_prover();
    let proof = prover.prove_editing(&input).unwrap();

    assert!(
        proof.public_inputs.editing_verified,
        "rotate re-execution should be verified"
    );
    assert_eq!(
        proof.public_inputs.operations_applied,
        vec![EditOperation::Rotate]
    );
    assert_eq!(proof.metadata.prover_type, "pico");
    println!(
        "Pico rotate re-execution proof generated in {}ms ({} bytes)",
        proof.metadata.generation_time_ms,
        proof.proof_bytes.len()
    );
}

// ---------------------------------------------------------------------------
// Chain: crop → resize → rotate
// ---------------------------------------------------------------------------

#[test]
fn test_pico_chain_crop_resize_rotate() {
    let original = make_test_image(16, 16);
    let orig_hash = sha256_hex(&original);

    // Step 1: Crop
    let crop_result = operations::crop(
        &original,
        &CropParams {
            x: 0,
            y: 0,
            width: 12,
            height: 12,
        },
    )
    .unwrap();
    let cropped_hash = sha256_hex(&crop_result.image_bytes);

    // Step 2: Resize
    let resize_result = operations::resize(
        &crop_result.image_bytes,
        &ResizeParams {
            width: 8,
            height: 8,
        },
    )
    .unwrap();
    let resized_hash = sha256_hex(&resize_result.image_bytes);

    // Step 3: Rotate
    let rotate_result = operations::rotate(
        &resize_result.image_bytes,
        &RotateParams {
            angle: RotationAngle::Deg180,
        },
    )
    .unwrap();
    let rotated_hash = sha256_hex(&rotate_result.image_bytes);

    let input = EditingProofInput {
        original_image_hash: orig_hash.clone(),
        edited_image_hash: rotated_hash.clone(),
        editing_records: vec![
            EditingRecordInput {
                operation: EditOperation::Crop,
                parameters: serde_json::json!({
                    "x": 0, "y": 0, "width": 12, "height": 12
                }),
                input_hash: orig_hash,
                output_hash: cropped_hash.clone(),
                raw_pixels: None,
                pixel_width: None,
                pixel_height: None,
            },
            EditingRecordInput {
                operation: EditOperation::Resize,
                parameters: serde_json::json!({"width": 8, "height": 8}),
                input_hash: cropped_hash,
                output_hash: resized_hash.clone(),
                raw_pixels: None,
                pixel_width: None,
                pixel_height: None,
            },
            EditingRecordInput {
                operation: EditOperation::Rotate,
                parameters: serde_json::json!({"angle": 180}),
                input_hash: resized_hash,
                output_hash: rotated_hash,
                raw_pixels: None,
                pixel_width: None,
                pixel_height: None,
            },
        ],
    };

    let prover = pico_prover();
    let proof = prover.prove_editing(&input).unwrap();

    assert!(proof.public_inputs.editing_verified, "editing should be verified");
    assert_eq!(proof.public_inputs.operations_applied.len(), 3);
    assert_eq!(
        proof.public_inputs.operations_applied,
        vec![EditOperation::Crop, EditOperation::Resize, EditOperation::Rotate]
    );
    assert_eq!(proof.metadata.prover_type, "pico");
    println!(
        "Pico chain proof (crop→resize→rotate) generated in {}ms ({} bytes)",
        proof.metadata.generation_time_ms,
        proof.proof_bytes.len()
    );
}

// ---------------------------------------------------------------------------
// Real image from disk: crop → resize → rotate
// ---------------------------------------------------------------------------

#[test]
fn test_pico_real_image_edit_and_prove() {
    let img_path = std::path::Path::new("imgs/test_img_small.JPG");
    if !img_path.exists() {
        eprintln!("Skipping: imgs/test_img_small.JPG not found");
        return;
    }

    let original = std::fs::read(img_path).unwrap();
    let orig_hash = sha256_hex(&original);
    println!("Original image: {} bytes, hash: {}", original.len(), &orig_hash[..16]);

    // Step 1: Crop center 1000x1000
    let crop_result = operations::crop(
        &original,
        &CropParams {
            x: 500,
            y: 500,
            width: 1000,
            height: 1000,
        },
    )
    .unwrap();
    let cropped_hash = sha256_hex(&crop_result.image_bytes);
    println!("After crop: {} bytes", crop_result.image_bytes.len());

    // Step 2: Resize to 256x256
    let resize_result = operations::resize(
        &crop_result.image_bytes,
        &ResizeParams {
            width: 256,
            height: 256,
        },
    )
    .unwrap();
    let resized_hash = sha256_hex(&resize_result.image_bytes);
    println!("After resize: {} bytes", resize_result.image_bytes.len());

    // Step 3: Rotate 90°
    let rotate_result = operations::rotate(
        &resize_result.image_bytes,
        &RotateParams {
            angle: RotationAngle::Deg90,
        },
    )
    .unwrap();
    let rotated_hash = sha256_hex(&rotate_result.image_bytes);
    println!("After rotate: {} bytes", rotate_result.image_bytes.len());

    let input = EditingProofInput {
        original_image_hash: orig_hash.clone(),
        edited_image_hash: rotated_hash.clone(),
        editing_records: vec![
            EditingRecordInput {
                operation: EditOperation::Crop,
                parameters: serde_json::json!({
                    "x": 500, "y": 500, "width": 1000, "height": 1000
                }),
                input_hash: orig_hash,
                output_hash: cropped_hash.clone(),
                raw_pixels: None,
                pixel_width: None,
                pixel_height: None,
            },
            EditingRecordInput {
                operation: EditOperation::Resize,
                parameters: serde_json::json!({"width": 256, "height": 256}),
                input_hash: cropped_hash,
                output_hash: resized_hash.clone(),
                raw_pixels: None,
                pixel_width: None,
                pixel_height: None,
            },
            EditingRecordInput {
                operation: EditOperation::Rotate,
                parameters: serde_json::json!({"angle": 90}),
                input_hash: resized_hash,
                output_hash: rotated_hash,
                raw_pixels: None,
                pixel_width: None,
                pixel_height: None,
            },
        ],
    };

    let prover = pico_prover();
    let proof = prover.prove_editing(&input).unwrap();

    assert!(proof.public_inputs.editing_verified, "editing should be verified");
    assert_eq!(proof.public_inputs.operations_applied.len(), 3);
    assert_eq!(
        proof.public_inputs.operations_applied,
        vec![EditOperation::Crop, EditOperation::Resize, EditOperation::Rotate]
    );
    assert_eq!(proof.metadata.prover_type, "pico");
    println!(
        "Pico real-image proof (crop→resize→rotate) generated in {}ms ({} bytes)",
        proof.metadata.generation_time_ms,
        proof.proof_bytes.len()
    );
}
