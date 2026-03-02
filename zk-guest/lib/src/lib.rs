//! Shared types between the Pico ZKVM guest (app/) and the host prover.
//!
//! This crate defines:
//! - [`CircuitInput`] — serialized input written to the ZKVM stdin
//! - [`PublicValuesStruct`] — ABI-encoded public values committed by the guest
//!
//! Both the guest program and the host prover depend on this crate
//! to ensure type-safe serialization across the ZKVM boundary.

use alloy_sol_types::sol;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Public values (committed to the proof, ABI-encoded for on-chain use)
// ---------------------------------------------------------------------------

sol! {
    /// Public values committed by the ZK guest program.
    ///
    /// These are the only values visible to a verifier:
    /// - `c2pa_verified`: 1 if C2PA provenance was verified, 0 otherwise
    /// - `editing_verified`: 1 if editing operations were verified, 0 otherwise
    /// - `original_image_hash`: SHA-256 hash of the original image (32 bytes)
    /// - `edited_image_hash`: SHA-256 hash of the edited image (32 bytes, zeroed if no edits)
    /// - `num_operations`: number of editing operations applied
    struct PublicValuesStruct {
        uint8 c2pa_verified;
        uint8 editing_verified;
        bytes32 original_image_hash;
        bytes32 edited_image_hash;
        uint32 num_operations;
    }
}

// ---------------------------------------------------------------------------
// Circuit input (serialized to ZKVM stdin via bincode)
// ---------------------------------------------------------------------------

/// Complete input for the ZK guest program.
///
/// Serialized with `bincode` and written to the ZKVM stdin
/// via `stdin_builder.write(&input)`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitInput {
    /// C2PA verification data (None if no C2PA metadata)
    pub c2pa_data: Option<C2paInputData>,
    /// SHA-256 hash of the original image (hex-encoded, 64 chars)
    pub original_image_hash: String,
    /// Editing operation records (empty if no edits)
    pub editing_records: Vec<EditingRecordData>,
    /// SHA-256 hash of the final edited image (None if no edits)
    pub edited_image_hash: Option<String>,
}

/// C2PA data passed into the ZKVM for structural verification.
///
/// Cryptographic signature verification (ECDSA, RSA-PSS, Ed25519, etc.)
/// is performed by the host using the `c2pa` library before the data
/// is passed to the guest.  The guest only validates the structural
/// integrity of the manifest data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C2paInputData {
    /// Active manifest label (URN identifier)
    pub active_manifest: String,
    /// Claim generator identifier
    pub claim_generator: String,
    /// Signing algorithm (e.g., "Es256", "PS256")
    pub algorithm: Option<String>,
}

/// A single editing operation record for the ZKVM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditingRecordData {
    /// Operation type: "crop", "resize", or "rotate"
    pub operation: String,
    /// Crop: x, y, width, height; Resize: width, height; Rotate: angle
    pub params: OperationParams,
    /// SHA-256 hash of the image before this operation (hex)
    pub input_hash: String,
    /// SHA-256 hash of the image after this operation (hex)
    pub output_hash: String,
}

/// Operation-specific parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationParams {
    /// Crop parameters
    Crop {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    },
    /// Resize parameters
    Resize { width: u32, height: u32 },
    /// Rotate parameters (angle: 90, 180, or 270)
    Rotate { angle: u32 },
}

// ---------------------------------------------------------------------------
// Helper: load ELF binary
// ---------------------------------------------------------------------------

/// Load the compiled RISC-V ELF binary from disk.
///
/// Used by the host prover to load the guest program.
pub fn load_elf(path: &str) -> Vec<u8> {
    std::fs::read(path).unwrap_or_else(|err| {
        panic!("Failed to load ELF file from {path}: {err}");
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_sol_types::SolValue;

    #[test]
    fn test_public_values_abi_encode_decode() {
        let pv = PublicValuesStruct {
            c2pa_verified: 1,
            editing_verified: 1,
            original_image_hash: [0xAA; 32].into(),
            edited_image_hash: [0xBB; 32].into(),
            num_operations: 2,
        };

        let encoded = pv.abi_encode();
        let decoded =
            PublicValuesStruct::abi_decode(&encoded, true).expect("ABI decode should succeed");

        assert_eq!(decoded.c2pa_verified, 1);
        assert_eq!(decoded.editing_verified, 1);
        assert_eq!(decoded.num_operations, 2);
    }

    #[test]
    fn test_circuit_input_serde_roundtrip() {
        let input = CircuitInput {
            c2pa_data: Some(C2paInputData {
                active_manifest: "urn:c2pa:test:1".to_string(),
                claim_generator: "TestCam/1.0".to_string(),
                algorithm: Some("Es256".to_string()),
            }),
            original_image_hash: "aa".repeat(32),
            editing_records: vec![EditingRecordData {
                operation: "crop".to_string(),
                params: OperationParams::Crop {
                    x: 10,
                    y: 20,
                    width: 100,
                    height: 80,
                },
                input_hash: "aa".repeat(32),
                output_hash: "bb".repeat(32),
            }],
            edited_image_hash: Some("bb".repeat(32)),
        };

        let json = serde_json::to_string(&input).unwrap();
        let parsed: CircuitInput = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.original_image_hash, input.original_image_hash);
        assert_eq!(parsed.editing_records.len(), 1);
    }
}
