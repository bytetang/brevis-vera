//! Data types for the ZK Proof Generation Layer.
//!
//! Defines proof inputs, outputs, public inputs, and error types
//! used throughout the ZK proof pipeline.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::editor::types::EditOperation;
use crate::provenance::types::C2paVerificationData;

// ---------------------------------------------------------------------------
// Proof inputs
// ---------------------------------------------------------------------------

/// Input for C2PA-only proof generation.
///
/// Proves that the original image has valid C2PA provenance
/// without revealing signer identity or certificate details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C2paProofInput {
    /// C2PA verification data (signature info, assertions, manifest)
    pub c2pa_data: C2paVerificationData,
    /// SHA-256 hash of the original image (hex-encoded)
    pub image_hash: String,
}

/// Input for editing-only proof generation.
///
/// Proves that the claimed editing operations were applied correctly
/// without revealing exact parameters (e.g., crop coordinates).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditingProofInput {
    /// SHA-256 hash of the original image (hex-encoded)
    pub original_image_hash: String,
    /// SHA-256 hash of the edited image (hex-encoded)
    pub edited_image_hash: String,
    /// Editing records describing applied operations
    pub editing_records: Vec<EditingRecordInput>,
}

/// A single editing record input for proof generation.
///
/// Mirrors [`crate::editor::types::EditingRecord`] but uses
/// typed parameters for circuit verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditingRecordInput {
    /// Type of editing operation
    pub operation: EditOperation,
    /// Operation-specific parameters (JSON value)
    pub parameters: serde_json::Value,
    /// SHA-256 hash of the image before this operation
    pub input_hash: String,
    /// SHA-256 hash of the image after this operation
    pub output_hash: String,
}

/// Input for combined proof generation (C2PA + editing).
///
/// Generates a single ZK proof that verifies both:
/// 1. The original image has valid C2PA provenance
/// 2. The editing operations were applied correctly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinedProofInput {
    /// C2PA verification data (`None` if no C2PA data available)
    pub c2pa_data: Option<C2paVerificationData>,
    /// SHA-256 hash of the original image (hex-encoded)
    pub original_image_hash: String,
    /// Editing records (may be empty if no edits were applied)
    pub editing_records: Vec<EditingRecordInput>,
    /// SHA-256 hash of the final edited image (`None` if no edits)
    pub edited_image_hash: Option<String>,
}

// ---------------------------------------------------------------------------
// Proof outputs
// ---------------------------------------------------------------------------

/// Public inputs visible in the ZK proof.
///
/// These are the values that a verifier can inspect without
/// learning any private details about the image or edits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicInputs {
    /// Whether C2PA provenance was verified
    pub c2pa_verified: bool,
    /// Whether editing operations were verified
    pub editing_verified: bool,
    /// SHA-256 hash of the original image
    pub original_image_hash: String,
    /// SHA-256 hash of the final edited image (if edits were applied)
    pub edited_image_hash: Option<String>,
    /// List of editing operations that were applied (types only, no params)
    pub operations_applied: Vec<EditOperation>,
}

/// Metadata about proof generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofMetadata {
    /// Type of prover used (e.g., "simulated", "pico")
    pub prover_type: String,
    /// Proof generation time in milliseconds
    pub generation_time_ms: u64,
    /// Timestamp of proof generation (ISO 8601)
    pub timestamp: String,
    /// Proof format version
    pub proof_version: String,
}

/// A generated ZK proof.
///
/// Contains the proof bytes, public inputs visible to verifiers,
/// and metadata about the proof generation process.
///
/// # Proof Format
///
/// The `proof_bytes` field contains the serialized proof.
/// For the simulated prover, this is a HMAC-SHA256 commitment.
/// For Pico ZKVM, this would be the actual STARK/SNARK proof.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProof {
    /// Serialized proof bytes
    pub proof_bytes: Vec<u8>,
    /// Public inputs visible to the verifier
    pub public_inputs: PublicInputs,
    /// Proof generation metadata
    pub metadata: ProofMetadata,
}

// ---------------------------------------------------------------------------
// Circuit verification results (internal)
// ---------------------------------------------------------------------------

/// Result of C2PA circuit verification.
#[derive(Debug, Clone)]
pub struct C2paCircuitResult {
    /// Whether the C2PA data structure is valid
    pub valid: bool,
    /// Reason for invalidity (if applicable)
    pub reason: Option<String>,
}

/// Result of editing circuit verification.
#[derive(Debug, Clone)]
pub struct EditingCircuitResult {
    /// Whether all editing operations were verified
    pub valid: bool,
    /// Per-operation results
    pub operation_results: Vec<OperationVerifyResult>,
}

/// Result of verifying a single editing operation.
#[derive(Debug, Clone)]
pub struct OperationVerifyResult {
    /// The operation type
    pub operation: EditOperation,
    /// Whether this operation passed verification
    pub valid: bool,
    /// Reason for failure (if applicable)
    pub reason: Option<String>,
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Errors that can occur during ZK proof generation or verification.
#[derive(Error, Debug)]
pub enum ZkError {
    /// Missing required input data
    #[error("Missing input: {0}")]
    MissingInput(String),

    /// Invalid proof input data
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// C2PA verification failed inside the circuit
    #[error("C2PA verification failed: {0}")]
    C2paVerificationFailed(String),

    /// Editing verification failed inside the circuit
    #[error("Editing verification failed: {0}")]
    EditingVerificationFailed(String),

    /// Hash mismatch between expected and computed values
    #[error("Hash mismatch: {field} — expected {expected}, got {actual}")]
    HashMismatch {
        /// Which field had the mismatch
        field: String,
        /// Expected hash value
        expected: String,
        /// Actual computed hash value
        actual: String,
    },

    /// Proof serialization/deserialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Proof generation failed (ZKVM error)
    #[error("Proof generation failed: {0}")]
    ProofGenerationFailed(String),

    /// Proof verification failed
    #[error("Proof verification failed: {0}")]
    VerificationFailed(String),

    /// Prover backend error
    #[error("Prover error: {0}")]
    ProverError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c2pa_proof_input_serialization() {
        let input = C2paProofInput {
            c2pa_data: C2paVerificationData {
                signature_info: None,
                assertions: vec![],
                active_manifest: "urn:test".to_string(),
                claim_generator: "test_gen".to_string(),
                ecdsa_signature: None,
                public_key: None,
            },
            image_hash: "abcd1234".to_string(),
        };

        let json = serde_json::to_string(&input).unwrap();
        let parsed: C2paProofInput = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.image_hash, "abcd1234");
        assert_eq!(parsed.c2pa_data.active_manifest, "urn:test");
    }

    #[test]
    fn test_combined_proof_input_serialization() {
        let input = CombinedProofInput {
            c2pa_data: None,
            original_image_hash: "hash_orig".to_string(),
            editing_records: vec![EditingRecordInput {
                operation: EditOperation::Crop,
                parameters: serde_json::json!({"x": 0, "y": 0, "width": 100, "height": 100}),
                input_hash: "hash_in".to_string(),
                output_hash: "hash_out".to_string(),
            }],
            edited_image_hash: Some("hash_edited".to_string()),
        };

        let json = serde_json::to_string(&input).unwrap();
        let parsed: CombinedProofInput = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.editing_records.len(), 1);
        assert_eq!(parsed.editing_records[0].operation, EditOperation::Crop);
    }

    #[test]
    fn test_zk_proof_serialization() {
        let proof = ZkProof {
            proof_bytes: vec![1, 2, 3, 4],
            public_inputs: PublicInputs {
                c2pa_verified: true,
                editing_verified: false,
                original_image_hash: "aabbcc".to_string(),
                edited_image_hash: None,
                operations_applied: vec![],
            },
            metadata: ProofMetadata {
                prover_type: "simulated".to_string(),
                generation_time_ms: 42,
                timestamp: "2026-01-01T00:00:00Z".to_string(),
                proof_version: "0.1.0".to_string(),
            },
        };

        let json = serde_json::to_string_pretty(&proof).unwrap();
        let parsed: ZkProof = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.proof_bytes, vec![1, 2, 3, 4]);
        assert!(parsed.public_inputs.c2pa_verified);
        assert!(!parsed.public_inputs.editing_verified);
    }

    #[test]
    fn test_public_inputs_with_operations() {
        let inputs = PublicInputs {
            c2pa_verified: true,
            editing_verified: true,
            original_image_hash: "orig".to_string(),
            edited_image_hash: Some("edited".to_string()),
            operations_applied: vec![EditOperation::Crop, EditOperation::Resize],
        };

        let json = serde_json::to_string(&inputs).unwrap();
        assert!(json.contains("\"crop\""));
        assert!(json.contains("\"resize\""));
    }

    #[test]
    fn test_zk_error_display() {
        let err = ZkError::HashMismatch {
            field: "image_hash".to_string(),
            expected: "aabb".to_string(),
            actual: "ccdd".to_string(),
        };
        let msg = format!("{err}");
        assert!(msg.contains("Hash mismatch"));
        assert!(msg.contains("aabb"));
        assert!(msg.contains("ccdd"));
    }

    #[test]
    fn test_editing_record_input_roundtrip() {
        let record = EditingRecordInput {
            operation: EditOperation::Resize,
            parameters: serde_json::json!({"width": 200, "height": 150}),
            input_hash: "input".to_string(),
            output_hash: "output".to_string(),
        };

        let json = serde_json::to_value(&record).unwrap();
        let parsed: EditingRecordInput = serde_json::from_value(json).unwrap();
        assert_eq!(parsed.operation, EditOperation::Resize);
        assert_eq!(parsed.output_hash, "output");
    }
}
