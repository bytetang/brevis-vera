//! ZK circuit verification logic.
//!
//! Contains the verification functions that would run inside the ZKVM circuit.
//! These functions implement the core logic for:
//!
//! 1. **C2PA Verification** — validates C2PA provenance data structure
//!    (cryptographic signature verification is done host-side by the `c2pa` library)
//!
//! 2. **Crop Verification** — proves a crop was applied correctly by
//!    verifying hash chain consistency
//!
//! 3. **Resize Verification** — proves a resize was applied with valid dimensions
//!
//! 4. **Rotate Verification** — proves a rotation was applied with valid angle
//!
//! 5. **Combined Verification** — runs all applicable checks in one pass
//!
//! # ZKVM Integration
//!
//! In a real Pico ZKVM deployment, these functions would be compiled to
//! RISC-V and executed inside the ZKVM. The simulated prover runs them
//! natively for development and testing.

use crate::editor::types::{EditOperation, sha256_hex};
use crate::provenance::types::C2paVerificationData;

use super::types::{
    C2paCircuitResult, C2paProofInput, CombinedProofInput, EditingCircuitResult,
    EditingProofInput, EditingRecordInput, OperationVerifyResult,
};

// ---------------------------------------------------------------------------
// C2PA verification circuit
// ---------------------------------------------------------------------------

/// Verify C2PA provenance data inside the circuit.
///
/// Validates that the C2PA data has the required structure for
/// ECDSA P-256 signature verification:
/// - Active manifest label is present and non-empty
/// - Claim generator is present and non-empty
/// - Signature info contains required fields
///
/// # ZKVM Note
///
/// In production (Pico ZKVM), this function would also:
/// 1. Verify the ECDSA P-256 signature using `brevis-network/signatures`
/// 2. Validate the certificate chain
/// 3. Check signature over the image hash
///
/// The simulated version validates structure only, sufficient for
/// testing the full pipeline.
pub fn verify_c2pa(input: &C2paProofInput) -> C2paCircuitResult {
    // Validate image hash is non-empty and looks like hex
    if input.image_hash.is_empty() {
        return C2paCircuitResult {
            valid: false,
            reason: Some("Image hash is empty".to_string()),
        };
    }

    if !input.image_hash.chars().all(|c| c.is_ascii_hexdigit()) {
        return C2paCircuitResult {
            valid: false,
            reason: Some("Image hash is not valid hex".to_string()),
        };
    }

    // Validate C2PA data structure
    let c2pa = &input.c2pa_data;

    if c2pa.active_manifest.is_empty() {
        return C2paCircuitResult {
            valid: false,
            reason: Some("Active manifest label is empty".to_string()),
        };
    }

    if c2pa.claim_generator.is_empty() {
        return C2paCircuitResult {
            valid: false,
            reason: Some("Claim generator is empty".to_string()),
        };
    }

    // Validate signature info (required for ECDSA verification)
    match &c2pa.signature_info {
        Some(sig_info) => {
            // Check signing algorithm is ECDSA P-256 (Es256)
            if let Some(ref alg) = sig_info.alg {
                let alg_upper = alg.to_uppercase();
                if alg_upper != "ES256"
                    && alg_upper != "PS256"
                    && alg_upper != "PS384"
                    && alg_upper != "PS512"
                {
                    return C2paCircuitResult {
                        valid: false,
                        reason: Some(format!("Unsupported signing algorithm: {alg}")),
                    };
                }
            }

            // In production ZKVM: verify ECDSA signature here using
            // brevis-network/signatures P-256 circuit
        }
        None => {
            // No signature info — still structurally valid but note it
            // In production, this would fail ECDSA verification
        }
    }

    C2paCircuitResult {
        valid: true,
        reason: None,
    }
}

/// Verify C2PA data structure directly (helper for combined proofs).
pub fn verify_c2pa_data(c2pa: &C2paVerificationData, image_hash: &str) -> C2paCircuitResult {
    let input = C2paProofInput {
        c2pa_data: c2pa.clone(),
        image_hash: image_hash.to_string(),
    };
    verify_c2pa(&input)
}

// ---------------------------------------------------------------------------
// Editing verification circuits
// ---------------------------------------------------------------------------

/// Verify a single crop operation.
///
/// When `raw_pixels`, `pixel_width`, and `pixel_height` are provided,
/// performs **full re-execution verification**:
/// 1. SHA-256(raw_pixels) must equal `input_hash`
/// 2. Extract the sub-rectangle defined by (x, y, width, height)
/// 3. SHA-256(cropped_pixels) must equal `output_hash`
///
/// This is the same logic that runs inside the ZKVM guest.
///
/// When raw pixels are NOT provided (backward compat), falls back to
/// parameter-only validation (non-zero dimensions, coordinates present,
/// hashes differ). This is NOT a cryptographic proof — it only checks
/// structural consistency.
fn verify_crop(record: &EditingRecordInput) -> OperationVerifyResult {
    let params = &record.parameters;

    // Check required parameters
    let width = params.get("width").and_then(|v| v.as_u64());
    let height = params.get("height").and_then(|v| v.as_u64());

    if width.is_none() || height.is_none() {
        return OperationVerifyResult {
            operation: EditOperation::Crop,
            valid: false,
            reason: Some("Missing width or height in crop parameters".to_string()),
        };
    }

    let w = width.unwrap();
    let h = height.unwrap();

    if w == 0 || h == 0 {
        return OperationVerifyResult {
            operation: EditOperation::Crop,
            valid: false,
            reason: Some("Crop dimensions must be non-zero".to_string()),
        };
    }

    // x and y must be present
    let x = params.get("x").and_then(|v| v.as_u64());
    let y = params.get("y").and_then(|v| v.as_u64());
    if x.is_none() || y.is_none() {
        return OperationVerifyResult {
            operation: EditOperation::Crop,
            valid: false,
            reason: Some("Missing x or y coordinate in crop parameters".to_string()),
        };
    }

    let cx = x.unwrap() as u32;
    let cy = y.unwrap() as u32;
    let cw = w as u32;
    let ch = h as u32;

    // --- Re-execution path (when raw pixels are available) ---
    if let (Some(raw_pixels), Some(pixel_w), Some(pixel_h)) =
        (record.raw_pixels.as_ref(), record.pixel_width, record.pixel_height)
    {
        // Validate pixel data size
        let expected_size = (pixel_w as usize) * (pixel_h as usize) * 4;
        if raw_pixels.len() != expected_size {
            return OperationVerifyResult {
                operation: EditOperation::Crop,
                valid: false,
                reason: Some(format!(
                    "Raw pixel data size mismatch: expected {} bytes ({}x{}x4), got {}",
                    expected_size, pixel_w, pixel_h, raw_pixels.len()
                )),
            };
        }

        // Bounds checking
        if cx + cw > pixel_w || cy + ch > pixel_h {
            return OperationVerifyResult {
                operation: EditOperation::Crop,
                valid: false,
                reason: Some(format!(
                    "Crop region ({},{} {}x{}) exceeds image bounds ({}x{})",
                    cx, cy, cw, ch, pixel_w, pixel_h
                )),
            };
        }

        // Step 1: Verify input hash = SHA-256(raw_pixels)
        let computed_input_hash = sha256_hex(raw_pixels);
        if computed_input_hash != record.input_hash {
            return OperationVerifyResult {
                operation: EditOperation::Crop,
                valid: false,
                reason: Some(format!(
                    "Input hash mismatch: computed {} but record claims {}",
                    computed_input_hash, record.input_hash
                )),
            };
        }

        // Step 2: Re-execute crop
        let cropped = crop_pixels_raw(raw_pixels, pixel_w, cx, cy, cw, ch);

        // Step 3: Verify output hash = SHA-256(cropped_pixels)
        let computed_output_hash = sha256_hex(&cropped);
        if computed_output_hash != record.output_hash {
            return OperationVerifyResult {
                operation: EditOperation::Crop,
                valid: false,
                reason: Some(format!(
                    "Output hash mismatch after re-execution: computed {} but record claims {}",
                    computed_output_hash, record.output_hash
                )),
            };
        }

        return OperationVerifyResult {
            operation: EditOperation::Crop,
            valid: true,
            reason: None,
        };
    }

    // --- Fallback: parameter-only validation (no raw pixels) ---
    // NOTE: This does NOT prove the crop was applied correctly.
    // It only checks structural consistency for backward compatibility.

    // Verify hash chain: input and output hashes must differ
    if record.input_hash == record.output_hash {
        return OperationVerifyResult {
            operation: EditOperation::Crop,
            valid: false,
            reason: Some("Input and output hashes are identical — crop must change the image".to_string()),
        };
    }

    OperationVerifyResult {
        operation: EditOperation::Crop,
        valid: true,
        reason: None,
    }
}

/// Extract a sub-rectangle from row-major RGBA pixel data.
///
/// This is the host-side equivalent of the ZKVM guest's `crop_pixels`
/// function. Both must produce identical output for the same input.
///
/// For each row in `[y..y+h]`, copies bytes `[x*4..(x+w)*4]` from
/// the source row into the output buffer.
fn crop_pixels_raw(pixels: &[u8], img_w: u32, x: u32, y: u32, w: u32, h: u32) -> Vec<u8> {
    let stride = (img_w as usize) * 4;
    let crop_row_bytes = (w as usize) * 4;
    let mut out = Vec::with_capacity((h as usize) * crop_row_bytes);

    for row in y..(y + h) {
        let row_start = (row as usize) * stride + (x as usize) * 4;
        let row_end = row_start + crop_row_bytes;
        out.extend_from_slice(&pixels[row_start..row_end]);
    }

    out
}

/// Verify a single resize operation.
///
/// Validates that the resize parameters are internally consistent:
/// - Width and height are non-zero
/// - Input and output hashes differ
///
/// # ZKVM Note
///
/// In production, the circuit would re-apply the resize using Lanczos3
/// interpolation and verify the output hash matches.
fn verify_resize(record: &EditingRecordInput) -> OperationVerifyResult {
    let params = &record.parameters;

    // The editor stores resize dimensions as "new_width"/"new_height",
    // but also accept "width"/"height" for direct API usage.
    let width = params.get("new_width").and_then(|v| v.as_u64())
        .or_else(|| params.get("width").and_then(|v| v.as_u64()));
    let height = params.get("new_height").and_then(|v| v.as_u64())
        .or_else(|| params.get("height").and_then(|v| v.as_u64()));

    if width.is_none() || height.is_none() {
        return OperationVerifyResult {
            operation: EditOperation::Resize,
            valid: false,
            reason: Some("Missing width or height in resize parameters".to_string()),
        };
    }

    if width.unwrap() == 0 || height.unwrap() == 0 {
        return OperationVerifyResult {
            operation: EditOperation::Resize,
            valid: false,
            reason: Some("Resize dimensions must be non-zero".to_string()),
        };
    }

    // Verify hash chain
    if record.input_hash == record.output_hash {
        return OperationVerifyResult {
            operation: EditOperation::Resize,
            valid: false,
            reason: Some(
                "Input and output hashes are identical — resize must change the image".to_string(),
            ),
        };
    }

    OperationVerifyResult {
        operation: EditOperation::Resize,
        valid: true,
        reason: None,
    }
}

/// Verify a single rotate operation.
///
/// Validates that the rotation angle is one of: 90, 180, 270.
///
/// # ZKVM Note
///
/// In production, the circuit would re-apply the rotation
/// and verify the output hash.
fn verify_rotate(record: &EditingRecordInput) -> OperationVerifyResult {
    let params = &record.parameters;

    let angle = params.get("angle").and_then(|v| v.as_str().or(v.as_u64().map(|_| "").into()));

    // Try to extract angle as string or number
    let angle_val: Option<u64> = params
        .get("angle")
        .and_then(|v| v.as_u64().or_else(|| v.as_str().and_then(|s| s.parse().ok())));

    if angle_val.is_none() && angle.is_none() {
        return OperationVerifyResult {
            operation: EditOperation::Rotate,
            valid: false,
            reason: Some("Missing angle in rotate parameters".to_string()),
        };
    }

    if let Some(a) = angle_val {
        if a != 90 && a != 180 && a != 270 {
            return OperationVerifyResult {
                operation: EditOperation::Rotate,
                valid: false,
                reason: Some(format!("Invalid rotation angle: {a} (must be 90, 180, or 270)")),
            };
        }
    } else {
        // Check string values like "90", "180", "270"
        let angle_str = params.get("angle").and_then(|v| v.as_str());
        match angle_str {
            Some("90") | Some("180") | Some("270") => {}
            Some(other) => {
                return OperationVerifyResult {
                    operation: EditOperation::Rotate,
                    valid: false,
                    reason: Some(format!("Invalid rotation angle: {other}")),
                };
            }
            None => {
                return OperationVerifyResult {
                    operation: EditOperation::Rotate,
                    valid: false,
                    reason: Some("Missing angle in rotate parameters".to_string()),
                };
            }
        }
    }

    // For 180° rotation: input and output may differ
    // For any rotation: hash chain must be valid
    if record.input_hash == record.output_hash {
        return OperationVerifyResult {
            operation: EditOperation::Rotate,
            valid: false,
            reason: Some(
                "Input and output hashes are identical — rotate must change the image".to_string(),
            ),
        };
    }

    OperationVerifyResult {
        operation: EditOperation::Rotate,
        valid: true,
        reason: None,
    }
}

/// Verify a single editing operation by dispatching to the appropriate circuit.
pub fn verify_operation(record: &EditingRecordInput) -> OperationVerifyResult {
    // Validate hashes are non-empty
    if record.input_hash.is_empty() || record.output_hash.is_empty() {
        return OperationVerifyResult {
            operation: record.operation,
            valid: false,
            reason: Some("Input or output hash is empty".to_string()),
        };
    }

    match record.operation {
        EditOperation::Crop => verify_crop(record),
        EditOperation::Resize => verify_resize(record),
        EditOperation::Rotate => verify_rotate(record),
    }
}

/// Verify all editing operations in an editing proof input.
///
/// Checks:
/// 1. Each operation is individually valid
/// 2. Hash chain is consistent (output of op N = input of op N+1)
/// 3. First input matches the original image hash
/// 4. Last output matches the edited image hash
pub fn verify_editing(input: &EditingProofInput) -> EditingCircuitResult {
    if input.editing_records.is_empty() {
        return EditingCircuitResult {
            valid: false,
            operation_results: vec![],
        };
    }

    let mut operation_results = Vec::new();
    let mut all_valid = true;

    // Verify hash chain starts with original image
    if input.editing_records[0].input_hash != input.original_image_hash {
        return EditingCircuitResult {
            valid: false,
            operation_results: vec![OperationVerifyResult {
                operation: input.editing_records[0].operation,
                valid: false,
                reason: Some(
                    "First operation input hash does not match original image hash".to_string(),
                ),
            }],
        };
    }

    // Verify each operation
    for (i, record) in input.editing_records.iter().enumerate() {
        let result = verify_operation(record);
        if !result.valid {
            all_valid = false;
        }
        operation_results.push(result);

        // Verify hash chain: output of op N = input of op N+1
        if i + 1 < input.editing_records.len()
            && record.output_hash != input.editing_records[i + 1].input_hash
        {
            all_valid = false;
            operation_results.push(OperationVerifyResult {
                operation: input.editing_records[i + 1].operation,
                valid: false,
                reason: Some(format!(
                    "Hash chain broken between operation {} and {}",
                    i,
                    i + 1
                )),
            });
        }
    }

    // Verify hash chain ends with edited image
    let last_output = &input.editing_records.last().unwrap().output_hash;
    if *last_output != input.edited_image_hash {
        all_valid = false;
        operation_results.push(OperationVerifyResult {
            operation: input.editing_records.last().unwrap().operation,
            valid: false,
            reason: Some("Last operation output hash does not match edited image hash".to_string()),
        });
    }

    EditingCircuitResult {
        valid: all_valid,
        operation_results,
    }
}

/// Run the combined verification circuit.
///
/// Verifies both C2PA provenance and editing operations in a single pass.
/// This produces the data needed for a combined ZK proof.
///
/// Returns `(c2pa_result, editing_result)` where either may be `None`
/// if the corresponding input data was not provided.
pub fn verify_combined(
    input: &CombinedProofInput,
) -> (Option<C2paCircuitResult>, Option<EditingCircuitResult>) {
    // C2PA verification (if data provided)
    let c2pa_result = input.c2pa_data.as_ref().map(|c2pa| {
        verify_c2pa_data(c2pa, &input.original_image_hash)
    });

    // Editing verification (if records provided)
    let editing_result = if !input.editing_records.is_empty() {
        let edited_hash = input.edited_image_hash.clone().unwrap_or_default();
        let editing_input = EditingProofInput {
            original_image_hash: input.original_image_hash.clone(),
            edited_image_hash: edited_hash,
            editing_records: input.editing_records.clone(),
        };
        Some(verify_editing(&editing_input))
    } else {
        None
    };

    (c2pa_result, editing_result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provenance::types::SignatureInfo;

    fn make_c2pa_input(alg: Option<&str>) -> C2paProofInput {
        C2paProofInput {
            c2pa_data: C2paVerificationData {
                signature_info: Some(SignatureInfo {
                    issuer: Some("CN=Test".to_string()),
                    time: Some("2026-01-01T00:00:00Z".to_string()),
                    cert_serial_number: Some("12345".to_string()),
                    alg: alg.map(String::from),
                    ecdsa_signature: None,
                    public_key: None,
                }),
                assertions: vec![],
                active_manifest: "urn:c2pa:test:1".to_string(),
                claim_generator: "TestCamera/1.0".to_string(),
                ecdsa_signature: None,
                public_key: None,
            },
            image_hash: "abcdef1234567890".to_string(),
        }
    }

    fn make_crop_record(input_hash: &str, output_hash: &str) -> EditingRecordInput {
        EditingRecordInput {
            operation: EditOperation::Crop,
            parameters: serde_json::json!({
                "x": 10, "y": 20, "width": 100, "height": 80
            }),
            input_hash: input_hash.to_string(),
            output_hash: output_hash.to_string(),
            raw_pixels: None,
            pixel_width: None,
            pixel_height: None,
        }
    }

    fn make_resize_record(input_hash: &str, output_hash: &str) -> EditingRecordInput {
        EditingRecordInput {
            operation: EditOperation::Resize,
            parameters: serde_json::json!({"width": 200, "height": 150}),
            input_hash: input_hash.to_string(),
            output_hash: output_hash.to_string(),
            raw_pixels: None,
            pixel_width: None,
            pixel_height: None,
        }
    }

    fn make_rotate_record(
        angle: &str,
        input_hash: &str,
        output_hash: &str,
    ) -> EditingRecordInput {
        EditingRecordInput {
            operation: EditOperation::Rotate,
            parameters: serde_json::json!({"angle": angle}),
            input_hash: input_hash.to_string(),
            output_hash: output_hash.to_string(),
            raw_pixels: None,
            pixel_width: None,
            pixel_height: None,
        }
    }

    // ---- C2PA circuit tests ----

    #[test]
    fn test_c2pa_valid_es256() {
        let input = make_c2pa_input(Some("Es256"));
        let result = verify_c2pa(&input);
        assert!(result.valid);
        assert!(result.reason.is_none());
    }

    #[test]
    fn test_c2pa_valid_no_sig_info() {
        let mut input = make_c2pa_input(None);
        input.c2pa_data.signature_info = None;
        let result = verify_c2pa(&input);
        assert!(result.valid); // structurally valid, sig verification in ZKVM
    }

    #[test]
    fn test_c2pa_empty_manifest() {
        let mut input = make_c2pa_input(Some("Es256"));
        input.c2pa_data.active_manifest = String::new();
        let result = verify_c2pa(&input);
        assert!(!result.valid);
        assert!(result.reason.unwrap().contains("manifest"));
    }

    #[test]
    fn test_c2pa_empty_claim_generator() {
        let mut input = make_c2pa_input(Some("Es256"));
        input.c2pa_data.claim_generator = String::new();
        let result = verify_c2pa(&input);
        assert!(!result.valid);
        assert!(result.reason.unwrap().contains("Claim generator"));
    }

    #[test]
    fn test_c2pa_unsupported_algorithm() {
        let input = make_c2pa_input(Some("EdDSA"));
        let result = verify_c2pa(&input);
        assert!(!result.valid);
        assert!(result.reason.unwrap().contains("Unsupported"));
    }

    #[test]
    fn test_c2pa_empty_image_hash() {
        let mut input = make_c2pa_input(Some("Es256"));
        input.image_hash = String::new();
        let result = verify_c2pa(&input);
        assert!(!result.valid);
    }

    #[test]
    fn test_c2pa_invalid_hex_hash() {
        let mut input = make_c2pa_input(Some("Es256"));
        input.image_hash = "not-hex!".to_string();
        let result = verify_c2pa(&input);
        assert!(!result.valid);
        assert!(result.reason.unwrap().contains("hex"));
    }

    // ---- Crop circuit tests ----

    #[test]
    fn test_crop_valid() {
        let record = make_crop_record("hash_a", "hash_b");
        let result = verify_operation(&record);
        assert!(result.valid);
    }

    #[test]
    fn test_crop_missing_width() {
        let record = EditingRecordInput {
            operation: EditOperation::Crop,
            parameters: serde_json::json!({"x": 0, "y": 0, "height": 100}),
            input_hash: "a".to_string(),
            output_hash: "b".to_string(),
            raw_pixels: None,
            pixel_width: None,
            pixel_height: None,
        };
        let result = verify_operation(&record);
        assert!(!result.valid);
        assert!(result.reason.unwrap().contains("width"));
    }

    #[test]
    fn test_crop_zero_dimension() {
        let record = EditingRecordInput {
            operation: EditOperation::Crop,
            parameters: serde_json::json!({"x": 0, "y": 0, "width": 0, "height": 100}),
            input_hash: "a".to_string(),
            output_hash: "b".to_string(),
            raw_pixels: None,
            pixel_width: None,
            pixel_height: None,
        };
        let result = verify_operation(&record);
        assert!(!result.valid);
        assert!(result.reason.unwrap().contains("non-zero"));
    }

    #[test]
    fn test_crop_same_hash() {
        let record = make_crop_record("same_hash", "same_hash");
        let result = verify_operation(&record);
        assert!(!result.valid);
        assert!(result.reason.unwrap().contains("identical"));
    }

    #[test]
    fn test_crop_missing_coordinates() {
        let record = EditingRecordInput {
            operation: EditOperation::Crop,
            parameters: serde_json::json!({"width": 100, "height": 100}),
            input_hash: "a".to_string(),
            output_hash: "b".to_string(),
            raw_pixels: None,
            pixel_width: None,
            pixel_height: None,
        };
        let result = verify_operation(&record);
        assert!(!result.valid);
        assert!(result.reason.unwrap().contains("coordinate"));
    }

    // ---- Resize circuit tests ----

    #[test]
    fn test_resize_valid() {
        let record = make_resize_record("hash_a", "hash_b");
        let result = verify_operation(&record);
        assert!(result.valid);
    }

    #[test]
    fn test_resize_zero_width() {
        let record = EditingRecordInput {
            operation: EditOperation::Resize,
            parameters: serde_json::json!({"width": 0, "height": 100}),
            input_hash: "a".to_string(),
            output_hash: "b".to_string(),
            raw_pixels: None,
            pixel_width: None,
            pixel_height: None,
        };
        let result = verify_operation(&record);
        assert!(!result.valid);
    }

    #[test]
    fn test_resize_same_hash() {
        let record = make_resize_record("same", "same");
        let result = verify_operation(&record);
        assert!(!result.valid);
    }

    // ---- Rotate circuit tests ----

    #[test]
    fn test_rotate_90_valid() {
        let record = make_rotate_record("90", "hash_a", "hash_b");
        let result = verify_operation(&record);
        assert!(result.valid);
    }

    #[test]
    fn test_rotate_180_valid() {
        let record = make_rotate_record("180", "hash_a", "hash_b");
        let result = verify_operation(&record);
        assert!(result.valid);
    }

    #[test]
    fn test_rotate_270_valid() {
        let record = make_rotate_record("270", "hash_a", "hash_b");
        let result = verify_operation(&record);
        assert!(result.valid);
    }

    #[test]
    fn test_rotate_invalid_angle() {
        let record = make_rotate_record("45", "hash_a", "hash_b");
        let result = verify_operation(&record);
        assert!(!result.valid);
        assert!(result.reason.unwrap().contains("Invalid"));
    }

    #[test]
    fn test_rotate_same_hash() {
        let record = make_rotate_record("90", "same", "same");
        let result = verify_operation(&record);
        assert!(!result.valid);
    }

    // ---- Empty hash tests ----

    #[test]
    fn test_empty_input_hash() {
        let record = EditingRecordInput {
            operation: EditOperation::Crop,
            parameters: serde_json::json!({"x": 0, "y": 0, "width": 100, "height": 100}),
            input_hash: "".to_string(),
            output_hash: "b".to_string(),
            raw_pixels: None,
            pixel_width: None,
            pixel_height: None,
        };
        let result = verify_operation(&record);
        assert!(!result.valid);
        assert!(result.reason.unwrap().contains("empty"));
    }

    // ---- Editing verification tests ----

    #[test]
    fn test_editing_single_crop() {
        let input = EditingProofInput {
            original_image_hash: "orig".to_string(),
            edited_image_hash: "cropped".to_string(),
            editing_records: vec![make_crop_record("orig", "cropped")],
        };
        let result = verify_editing(&input);
        assert!(result.valid);
        assert_eq!(result.operation_results.len(), 1);
    }

    #[test]
    fn test_editing_chain_crop_resize() {
        let input = EditingProofInput {
            original_image_hash: "orig".to_string(),
            edited_image_hash: "resized".to_string(),
            editing_records: vec![
                make_crop_record("orig", "cropped"),
                make_resize_record("cropped", "resized"),
            ],
        };
        let result = verify_editing(&input);
        assert!(result.valid);
        assert_eq!(result.operation_results.len(), 2);
    }

    #[test]
    fn test_editing_broken_chain() {
        let input = EditingProofInput {
            original_image_hash: "orig".to_string(),
            edited_image_hash: "final".to_string(),
            editing_records: vec![
                make_crop_record("orig", "cropped"),
                make_resize_record("wrong_hash", "final"), // chain broken
            ],
        };
        let result = verify_editing(&input);
        assert!(!result.valid);
    }

    #[test]
    fn test_editing_wrong_original() {
        let input = EditingProofInput {
            original_image_hash: "expected_orig".to_string(),
            edited_image_hash: "cropped".to_string(),
            editing_records: vec![make_crop_record("wrong_orig", "cropped")],
        };
        let result = verify_editing(&input);
        assert!(!result.valid);
    }

    #[test]
    fn test_editing_wrong_final() {
        let input = EditingProofInput {
            original_image_hash: "orig".to_string(),
            edited_image_hash: "expected_final".to_string(),
            editing_records: vec![make_crop_record("orig", "actual_final")],
        };
        let result = verify_editing(&input);
        assert!(!result.valid);
    }

    #[test]
    fn test_editing_empty_records() {
        let input = EditingProofInput {
            original_image_hash: "orig".to_string(),
            edited_image_hash: "same".to_string(),
            editing_records: vec![],
        };
        let result = verify_editing(&input);
        assert!(!result.valid);
    }

    // ---- Combined verification tests ----

    #[test]
    fn test_combined_c2pa_only() {
        let input = CombinedProofInput {
            c2pa_data: Some(C2paVerificationData {
                signature_info: Some(SignatureInfo {
                    issuer: Some("CN=Test".to_string()),
                    time: None,
                    cert_serial_number: None,
                    alg: Some("Es256".to_string()),
                    ecdsa_signature: None,
                    public_key: None,
                }),
                assertions: vec![],
                active_manifest: "urn:test".to_string(),
                claim_generator: "Test/1.0".to_string(),
                ecdsa_signature: None,
                public_key: None,
            }),
            original_image_hash: "abcdef".to_string(),
            editing_records: vec![],
            edited_image_hash: None,
        };

        let (c2pa, editing) = verify_combined(&input);
        assert!(c2pa.unwrap().valid);
        assert!(editing.is_none());
    }

    #[test]
    fn test_combined_editing_only() {
        let input = CombinedProofInput {
            c2pa_data: None,
            original_image_hash: "orig".to_string(),
            editing_records: vec![make_crop_record("orig", "cropped")],
            edited_image_hash: Some("cropped".to_string()),
        };

        let (c2pa, editing) = verify_combined(&input);
        assert!(c2pa.is_none());
        assert!(editing.unwrap().valid);
    }

    #[test]
    fn test_combined_both() {
        let input = CombinedProofInput {
            c2pa_data: Some(C2paVerificationData {
                signature_info: None,
                assertions: vec![],
                active_manifest: "urn:test".to_string(),
                claim_generator: "Test/1.0".to_string(),
                ecdsa_signature: None,
                public_key: None,
            }),
            original_image_hash: "abcdef".to_string(),
            editing_records: vec![make_crop_record("abcdef", "cropped")],
            edited_image_hash: Some("cropped".to_string()),
        };

        let (c2pa, editing) = verify_combined(&input);
        assert!(c2pa.unwrap().valid);
        assert!(editing.unwrap().valid);
    }

    // ---- Crop re-execution tests ----

    /// Create a simple 4x4 RGBA test image (64 bytes).
    /// Each pixel (r,g,b,a) encodes its position for easy verification.
    fn make_test_pixels(w: u32, h: u32) -> Vec<u8> {
        let mut pixels = Vec::with_capacity((w * h * 4) as usize);
        for y in 0..h {
            for x in 0..w {
                pixels.push(x as u8);       // R
                pixels.push(y as u8);       // G
                pixels.push((x + y) as u8); // B
                pixels.push(255);           // A
            }
        }
        pixels
    }

    #[test]
    fn test_crop_reexecution_valid() {
        // Create a 4x4 image, crop to (1,1, 2x2)
        let pixels = make_test_pixels(4, 4);
        let input_hash = sha256_hex(&pixels);

        let cropped = crop_pixels_raw(&pixels, 4, 1, 1, 2, 2);
        let output_hash = sha256_hex(&cropped);

        let record = EditingRecordInput {
            operation: EditOperation::Crop,
            parameters: serde_json::json!({
                "x": 1, "y": 1, "width": 2, "height": 2,
                "source_width": 4, "source_height": 4
            }),
            input_hash,
            output_hash,
            raw_pixels: Some(pixels),
            pixel_width: Some(4),
            pixel_height: Some(4),
        };

        let result = verify_operation(&record);
        assert!(result.valid, "Expected valid but got: {:?}", result.reason);
    }

    #[test]
    fn test_crop_reexecution_tampered_output_hash() {
        let pixels = make_test_pixels(4, 4);
        let input_hash = sha256_hex(&pixels);

        let record = EditingRecordInput {
            operation: EditOperation::Crop,
            parameters: serde_json::json!({
                "x": 1, "y": 1, "width": 2, "height": 2
            }),
            input_hash,
            output_hash: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            raw_pixels: Some(pixels),
            pixel_width: Some(4),
            pixel_height: Some(4),
        };

        let result = verify_operation(&record);
        assert!(!result.valid);
        assert!(result.reason.as_ref().unwrap().contains("Output hash mismatch"));
    }

    #[test]
    fn test_crop_reexecution_tampered_input_hash() {
        let pixels = make_test_pixels(4, 4);
        let cropped = crop_pixels_raw(&pixels, 4, 0, 0, 2, 2);
        let output_hash = sha256_hex(&cropped);

        let record = EditingRecordInput {
            operation: EditOperation::Crop,
            parameters: serde_json::json!({
                "x": 0, "y": 0, "width": 2, "height": 2
            }),
            input_hash: "aaaa".to_string(), // wrong input hash
            output_hash,
            raw_pixels: Some(pixels),
            pixel_width: Some(4),
            pixel_height: Some(4),
        };

        let result = verify_operation(&record);
        assert!(!result.valid);
        assert!(result.reason.as_ref().unwrap().contains("Input hash mismatch"));
    }

    #[test]
    fn test_crop_reexecution_out_of_bounds() {
        let pixels = make_test_pixels(4, 4);
        let input_hash = sha256_hex(&pixels);

        let record = EditingRecordInput {
            operation: EditOperation::Crop,
            parameters: serde_json::json!({
                "x": 3, "y": 3, "width": 2, "height": 2  // 3+2=5 > 4
            }),
            input_hash,
            output_hash: "anything".to_string(),
            raw_pixels: Some(pixels),
            pixel_width: Some(4),
            pixel_height: Some(4),
        };

        let result = verify_operation(&record);
        assert!(!result.valid);
        assert!(result.reason.as_ref().unwrap().contains("exceeds image bounds"));
    }

    #[test]
    fn test_crop_reexecution_pixel_size_mismatch() {
        // Provide pixel data that doesn't match declared dimensions
        let pixels = vec![0u8; 32]; // too small for 4x4
        let input_hash = sha256_hex(&pixels);

        let record = EditingRecordInput {
            operation: EditOperation::Crop,
            parameters: serde_json::json!({
                "x": 0, "y": 0, "width": 2, "height": 2
            }),
            input_hash,
            output_hash: "anything".to_string(),
            raw_pixels: Some(pixels),
            pixel_width: Some(4),
            pixel_height: Some(4),
        };

        let result = verify_operation(&record);
        assert!(!result.valid);
        assert!(result.reason.as_ref().unwrap().contains("size mismatch"));
    }

    #[test]
    fn test_crop_pixels_raw_correctness() {
        // 3x3 image, crop (1,1, 2x2)
        let pixels = make_test_pixels(3, 3);
        let cropped = crop_pixels_raw(&pixels, 3, 1, 1, 2, 2);

        // Expected: 2x2 = 16 bytes
        assert_eq!(cropped.len(), 16);

        // Pixel at (1,1) in original: R=1, G=1, B=2, A=255
        assert_eq!(&cropped[0..4], &[1, 1, 2, 255]);
        // Pixel at (2,1) in original: R=2, G=1, B=3, A=255
        assert_eq!(&cropped[4..8], &[2, 1, 3, 255]);
        // Pixel at (1,2) in original: R=1, G=2, B=3, A=255
        assert_eq!(&cropped[8..12], &[1, 2, 3, 255]);
        // Pixel at (2,2) in original: R=2, G=2, B=4, A=255
        assert_eq!(&cropped[12..16], &[2, 2, 4, 255]);
    }

    #[test]
    fn test_crop_fallback_without_pixels() {
        // Without raw_pixels, should use parameter-only validation (backward compat)
        let record = make_crop_record("hash_a", "hash_b");
        let result = verify_operation(&record);
        assert!(result.valid, "Fallback path should still pass for valid params");
    }
}
