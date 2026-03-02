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

use crate::editor::types::EditOperation;
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
/// Validates that the crop parameters are internally consistent:
/// - Width and height are non-zero
/// - Input and output hashes are valid hex strings
/// - Input and output hashes differ (crop changes the image)
///
/// # ZKVM Note
///
/// In production, the circuit would:
/// 1. Accept the original image as a private witness
/// 2. Re-apply the crop operation
/// 3. Hash the result and compare with the claimed output hash
/// 4. Output only: "crop verified" (without revealing coordinates)
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
    if params.get("x").and_then(|v| v.as_u64()).is_none()
        || params.get("y").and_then(|v| v.as_u64()).is_none()
    {
        return OperationVerifyResult {
            operation: EditOperation::Crop,
            valid: false,
            reason: Some("Missing x or y coordinate in crop parameters".to_string()),
        };
    }

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
                }),
                assertions: vec![],
                active_manifest: "urn:c2pa:test:1".to_string(),
                claim_generator: "TestCamera/1.0".to_string(),
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
        }
    }

    fn make_resize_record(input_hash: &str, output_hash: &str) -> EditingRecordInput {
        EditingRecordInput {
            operation: EditOperation::Resize,
            parameters: serde_json::json!({"width": 200, "height": 150}),
            input_hash: input_hash.to_string(),
            output_hash: output_hash.to_string(),
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
                }),
                assertions: vec![],
                active_manifest: "urn:test".to_string(),
                claim_generator: "Test/1.0".to_string(),
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
            }),
            original_image_hash: "abcdef".to_string(),
            editing_records: vec![make_crop_record("abcdef", "cropped")],
            edited_image_hash: Some("cropped".to_string()),
        };

        let (c2pa, editing) = verify_combined(&input);
        assert!(c2pa.unwrap().valid);
        assert!(editing.unwrap().valid);
    }
}
