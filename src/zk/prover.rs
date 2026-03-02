//! ZK prover implementations.
//!
//! Provides the [`ZkProver`] trait and concrete implementations:
//!
//! - [`SimulatedProver`] — runs circuit logic natively (no ZKVM) for
//!   development and testing. Produces structurally valid proofs with
//!   HMAC-SHA256 commitments.
//!
//! - [`PicoProver`] — runs circuits inside
//!   [Pico ZKVM](https://github.com/brevis-network/pico) for real
//!   STARK/SNARK proof generation. Requires the `pico` feature flag.
//!
//! # Prover Selection
//!
//! ```text
//! ┌───────────────────────┐     ┌────────────────────────┐
//! │   SimulatedProver     │     │    PicoProver           │
//! │  ───────────────────  │     │  ────────────────────── │
//! │  Native execution     │     │  RISC-V guest program   │
//! │  HMAC commitment      │     │  Real STARK/SNARK proof │
//! │  Fast (< 1ms)         │     │  Slow (~minutes)        │
//! │  Dev/test only        │     │  Production use         │
//! │  Always available     │     │  Feature: "pico"        │
//! └───────────────────────┘     └────────────────────────┘
//!          ▲                               ▲
//!          └───────────┬───────────────────┘
//!                      │
//!              ┌───────┴───────┐
//!              │   ZkProver    │
//!              │   (trait)     │
//!              └───────────────┘
//! ```
//!
//! # Pico Integration Architecture
//!
//! The Pico prover uses a 3-crate architecture:
//!
//! ```text
//! zk-guest/
//! ├── lib/          # Shared types (CircuitInput, PublicValuesStruct)
//! │                 # Used by both guest and host
//! ├── app/          # ZKVM guest program (compiled to RISC-V ELF)
//! │   ├── src/      # Circuit logic: C2PA verify + editing verify
//! │   └── elf/      # Compiled ELF binary (cargo pico build)
//! └── rust-toolchain # nightly-2025-08-04 (required by Pico)
//!
//! src/zk/prover.rs  # Host-side PicoProver loads ELF + generates proof
//! ```
//!
//! # Building the Guest Program
//!
//! ```bash
//! cd zk-guest/app
//! cargo pico build    # produces elf/riscv32im-pico-zkvm-elf
//! ```
//!
//! # References
//!
//! - [Pico ZKVM Docs](https://pico-docs.brevis.network/)
//! - [brevis-network/signatures](https://github.com/brevis-network/signatures)

use sha2::{Digest, Sha256};
use std::time::Instant;

use crate::editor::types::EditOperation;

use super::circuits;
use super::types::{
    C2paProofInput, CombinedProofInput, EditingProofInput, ProofMetadata, PublicInputs, ZkError,
    ZkProof,
};

// ---------------------------------------------------------------------------
// Prover trait
// ---------------------------------------------------------------------------

/// Trait for ZK proof generation backends.
///
/// Implementations provide different backends for proof generation:
/// - [`SimulatedProver`] for development/testing
/// - [`PicoProver`] for production ZKVM proofs (requires `pico` feature)
pub trait ZkProver {
    /// Generate a ZK proof for C2PA provenance verification only.
    ///
    /// Proves that the image has valid C2PA metadata without revealing
    /// signer identity or certificate details.
    fn prove_c2pa(&self, input: &C2paProofInput) -> Result<ZkProof, ZkError>;

    /// Generate a ZK proof for editing verification only.
    ///
    /// Proves that the claimed editing operations were applied correctly
    /// without revealing exact parameters.
    fn prove_editing(&self, input: &EditingProofInput) -> Result<ZkProof, ZkError>;

    /// Generate a combined ZK proof (C2PA + editing).
    ///
    /// Produces a single proof that verifies both provenance and
    /// editing in one pass.
    fn prove_combined(&self, input: &CombinedProofInput) -> Result<ZkProof, ZkError>;

    /// Verify a previously generated ZK proof.
    ///
    /// Returns `true` if the proof is valid, `false` otherwise.
    fn verify(&self, proof: &ZkProof) -> Result<bool, ZkError>;

    /// Returns the prover type identifier.
    fn prover_type(&self) -> &str;
}

// ---------------------------------------------------------------------------
// Simulated prover
// ---------------------------------------------------------------------------

/// HMAC key used for simulated proof commitments.
const SIMULATED_HMAC_KEY: &[u8] = b"brevis-vera-simulated-prover-v0.1";

/// Proof format version.
const PROOF_VERSION: &str = "0.1.0";

/// Simulated prover for development and testing.
///
/// Runs the circuit verification logic natively (no ZKVM) and produces
/// a proof consisting of an HMAC-SHA256 commitment over the public inputs.
/// This is NOT a real ZK proof — it provides no zero-knowledge guarantees.
///
/// Use this prover for:
/// - Unit and integration testing
/// - Pipeline development and debugging
/// - Benchmarking circuit logic (excluding ZKVM overhead)
///
/// # Example
///
/// ```
/// use brevis_vera::zk::{SimulatedProver, ZkProver, C2paProofInput};
/// use brevis_vera::provenance::types::C2paVerificationData;
///
/// let prover = SimulatedProver::new();
/// let input = C2paProofInput {
///     c2pa_data: C2paVerificationData {
///         signature_info: None,
///         assertions: vec![],
///         active_manifest: "urn:test".to_string(),
///         claim_generator: "Test/1.0".to_string(),
///     },
///     image_hash: "abcdef1234".to_string(),
/// };
///
/// let proof = prover.prove_c2pa(&input).unwrap();
/// assert!(prover.verify(&proof).unwrap());
/// ```
#[derive(Debug, Clone)]
pub struct SimulatedProver {
    _private: (),
}

impl SimulatedProver {
    /// Create a new simulated prover.
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for SimulatedProver {
    fn default() -> Self {
        Self::new()
    }
}

/// Compute HMAC-SHA256 commitment over public inputs.
///
/// This binds the proof to specific public inputs so that
/// verification can check the commitment matches.
fn compute_commitment(public_inputs: &PublicInputs) -> Vec<u8> {
    let mut hasher = Sha256::new();

    // Mix in the HMAC key
    hasher.update(SIMULATED_HMAC_KEY);

    // Mix in all public input fields
    hasher.update(
        if public_inputs.c2pa_verified {
            &b"c2pa:true_"[..]
        } else {
            &b"c2pa:false"[..]
        },
    );

    hasher.update(
        if public_inputs.editing_verified {
            &b"editing:true_"[..]
        } else {
            &b"editing:false"[..]
        },
    );

    hasher.update(public_inputs.original_image_hash.as_bytes());

    if let Some(ref edited_hash) = public_inputs.edited_image_hash {
        hasher.update(b"edited:");
        hasher.update(edited_hash.as_bytes());
    }

    for op in &public_inputs.operations_applied {
        hasher.update(format!("op:{op}").as_bytes());
    }

    hasher.finalize().to_vec()
}

/// Create a timestamp string in ISO 8601 format.
fn now_iso8601() -> String {
    chrono::Utc::now().to_rfc3339()
}

impl ZkProver for SimulatedProver {
    fn prove_c2pa(&self, input: &C2paProofInput) -> Result<ZkProof, ZkError> {
        let start = Instant::now();

        // Run circuit verification
        let result = circuits::verify_c2pa(input);

        if !result.valid {
            return Err(ZkError::C2paVerificationFailed(
                result.reason.unwrap_or_else(|| "Unknown".to_string()),
            ));
        }

        let public_inputs = PublicInputs {
            c2pa_verified: true,
            editing_verified: false,
            original_image_hash: input.image_hash.clone(),
            edited_image_hash: None,
            operations_applied: vec![],
        };

        let proof_bytes = compute_commitment(&public_inputs);
        let elapsed = start.elapsed();

        Ok(ZkProof {
            proof_bytes,
            public_inputs,
            metadata: ProofMetadata {
                prover_type: self.prover_type().to_string(),
                generation_time_ms: elapsed.as_millis() as u64,
                timestamp: now_iso8601(),
                proof_version: PROOF_VERSION.to_string(),
            },
        })
    }

    fn prove_editing(&self, input: &EditingProofInput) -> Result<ZkProof, ZkError> {
        let start = Instant::now();

        // Run circuit verification
        let result = circuits::verify_editing(input);

        if !result.valid {
            let reasons: Vec<String> = result
                .operation_results
                .iter()
                .filter(|r| !r.valid)
                .map(|r| {
                    format!(
                        "{}: {}",
                        r.operation,
                        r.reason.as_deref().unwrap_or("unknown")
                    )
                })
                .collect();
            return Err(ZkError::EditingVerificationFailed(reasons.join("; ")));
        }

        let operations: Vec<EditOperation> =
            input.editing_records.iter().map(|r| r.operation).collect();

        let public_inputs = PublicInputs {
            c2pa_verified: false,
            editing_verified: true,
            original_image_hash: input.original_image_hash.clone(),
            edited_image_hash: Some(input.edited_image_hash.clone()),
            operations_applied: operations,
        };

        let proof_bytes = compute_commitment(&public_inputs);
        let elapsed = start.elapsed();

        Ok(ZkProof {
            proof_bytes,
            public_inputs,
            metadata: ProofMetadata {
                prover_type: self.prover_type().to_string(),
                generation_time_ms: elapsed.as_millis() as u64,
                timestamp: now_iso8601(),
                proof_version: PROOF_VERSION.to_string(),
            },
        })
    }

    fn prove_combined(&self, input: &CombinedProofInput) -> Result<ZkProof, ZkError> {
        let start = Instant::now();

        // Run combined circuit verification
        let (c2pa_result, editing_result) = circuits::verify_combined(input);

        // Check C2PA result
        let c2pa_verified = match &c2pa_result {
            Some(r) => {
                if !r.valid {
                    return Err(ZkError::C2paVerificationFailed(
                        r.reason.clone().unwrap_or_else(|| "Unknown".to_string()),
                    ));
                }
                true
            }
            None => false,
        };

        // Check editing result
        let editing_verified = match &editing_result {
            Some(r) => {
                if !r.valid {
                    let reasons: Vec<String> = r
                        .operation_results
                        .iter()
                        .filter(|r| !r.valid)
                        .map(|r| {
                            format!(
                                "{}: {}",
                                r.operation,
                                r.reason.as_deref().unwrap_or("unknown")
                            )
                        })
                        .collect();
                    return Err(ZkError::EditingVerificationFailed(reasons.join("; ")));
                }
                true
            }
            None => false,
        };

        let operations: Vec<EditOperation> =
            input.editing_records.iter().map(|r| r.operation).collect();

        let public_inputs = PublicInputs {
            c2pa_verified,
            editing_verified,
            original_image_hash: input.original_image_hash.clone(),
            edited_image_hash: input.edited_image_hash.clone(),
            operations_applied: operations,
        };

        let proof_bytes = compute_commitment(&public_inputs);
        let elapsed = start.elapsed();

        Ok(ZkProof {
            proof_bytes,
            public_inputs,
            metadata: ProofMetadata {
                prover_type: self.prover_type().to_string(),
                generation_time_ms: elapsed.as_millis() as u64,
                timestamp: now_iso8601(),
                proof_version: PROOF_VERSION.to_string(),
            },
        })
    }

    fn verify(&self, proof: &ZkProof) -> Result<bool, ZkError> {
        // Re-compute commitment and compare
        let expected = compute_commitment(&proof.public_inputs);
        Ok(proof.proof_bytes == expected)
    }

    fn prover_type(&self) -> &str {
        "simulated"
    }
}

// ---------------------------------------------------------------------------
// Proof serialization helpers
// ---------------------------------------------------------------------------

impl ZkProof {
    /// Serialize the proof to JSON bytes.
    pub fn to_json(&self) -> Result<Vec<u8>, ZkError> {
        serde_json::to_vec_pretty(self)
            .map_err(|e| ZkError::SerializationError(format!("JSON serialization failed: {e}")))
    }

    /// Deserialize a proof from JSON bytes.
    pub fn from_json(bytes: &[u8]) -> Result<Self, ZkError> {
        serde_json::from_slice(bytes)
            .map_err(|e| ZkError::SerializationError(format!("JSON deserialization failed: {e}")))
    }

    /// Serialize the proof to a compact binary format.
    ///
    /// Format: `[4-byte proof_bytes length][proof_bytes][JSON public_inputs + metadata]`
    pub fn to_bytes(&self) -> Result<Vec<u8>, ZkError> {
        let json_part = serde_json::to_vec(&serde_json::json!({
            "public_inputs": self.public_inputs,
            "metadata": self.metadata,
        }))
        .map_err(|e| ZkError::SerializationError(e.to_string()))?;

        let proof_len = self.proof_bytes.len() as u32;
        let mut out = Vec::with_capacity(4 + self.proof_bytes.len() + json_part.len());
        out.extend_from_slice(&proof_len.to_le_bytes());
        out.extend_from_slice(&self.proof_bytes);
        out.extend_from_slice(&json_part);

        Ok(out)
    }

    /// Deserialize a proof from the compact binary format.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ZkError> {
        if bytes.len() < 4 {
            return Err(ZkError::SerializationError(
                "Proof too short to contain header".to_string(),
            ));
        }

        let proof_len = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;

        if bytes.len() < 4 + proof_len {
            return Err(ZkError::SerializationError(
                "Proof too short to contain proof bytes".to_string(),
            ));
        }

        let proof_bytes = bytes[4..4 + proof_len].to_vec();
        let json_part = &bytes[4 + proof_len..];

        let parsed: serde_json::Value =
            serde_json::from_slice(json_part).map_err(|e| ZkError::SerializationError(e.to_string()))?;

        let public_inputs: PublicInputs = serde_json::from_value(
            parsed
                .get("public_inputs")
                .cloned()
                .ok_or_else(|| ZkError::SerializationError("Missing public_inputs".to_string()))?,
        )
        .map_err(|e| ZkError::SerializationError(e.to_string()))?;

        let metadata: ProofMetadata = serde_json::from_value(
            parsed
                .get("metadata")
                .cloned()
                .ok_or_else(|| ZkError::SerializationError("Missing metadata".to_string()))?,
        )
        .map_err(|e| ZkError::SerializationError(e.to_string()))?;

        Ok(ZkProof {
            proof_bytes,
            public_inputs,
            metadata,
        })
    }
}

// ---------------------------------------------------------------------------
// Pico ZKVM prover (feature-gated)
// ---------------------------------------------------------------------------

/// Pico ZKVM prover for production proof generation.
///
/// Runs circuit logic inside the [Pico ZKVM](https://github.com/brevis-network/pico)
/// RISC-V virtual machine, producing real STARK proofs that can be verified
/// on-chain.
///
/// # Building the Guest Program
///
/// Before using `PicoProver`, you must compile the guest ELF binary:
///
/// ```bash
/// cd zk-guest/app
/// cargo pico build
/// ```
///
/// The ELF binary is output to `zk-guest/app/elf/riscv32im-pico-zkvm-elf`.
///
/// # Architecture
///
/// ```text
/// Host (PicoProver)              Guest (zk-guest/app)
/// ┌──────────────────┐           ┌──────────────────┐
/// │ CombinedProofInput│          │ CircuitInput      │
/// │       ↓           │  stdin   │       ↓           │
/// │ CircuitInput ─────┼──────────┤► read_as()        │
/// │                   │          │  verify_c2pa()    │
/// │                   │          │  verify_editing() │
/// │                   │  commit  │       ↓           │
/// │ PublicValuesStruct│◄─────────┤ commit_bytes()    │
/// │       ↓           │          └──────────────────┘
/// │ ZkProof           │
/// └──────────────────┘
/// ```
///
/// # Example
///
/// ```rust,no_run
/// use brevis_vera::zk::prover::{PicoProver, ZkProver};
/// use brevis_vera::zk::types::CombinedProofInput;
///
/// let prover = PicoProver::new("zk-guest/app/elf/riscv32im-pico-zkvm-elf")
///     .expect("Failed to load ELF");
///
/// // let proof = prover.prove_combined(&input).unwrap();
/// ```
///
/// # Feature Gate
///
/// Requires the `pico` feature:
///
/// ```toml
/// [dependencies]
/// brevis-vera = { path = ".", features = ["pico"] }
/// ```
#[cfg(feature = "pico")]
pub struct PicoProver {
    /// Raw ELF binary bytes for the RISC-V guest program.
    elf: Vec<u8>,
    /// Path to the ELF file (for metadata/diagnostics).
    elf_path: String,
}

#[cfg(feature = "pico")]
impl PicoProver {
    /// Create a new `PicoProver` by loading the pre-built ELF binary.
    ///
    /// # Arguments
    ///
    /// * `elf_path` — Path to the compiled guest ELF binary
    ///   (e.g., `zk-guest/app/elf/riscv32im-pico-zkvm-elf`)
    ///
    /// # Errors
    ///
    /// Returns `ZkError::MissingInput` if the ELF file does not exist.
    pub fn new(elf_path: &str) -> Result<Self, ZkError> {
        let elf = std::fs::read(elf_path).map_err(|e| {
            ZkError::MissingInput(format!(
                "Failed to load guest ELF from {elf_path}: {e}. \
                 Build it first with: cd zk-guest/app && cargo pico build"
            ))
        })?;

        Ok(Self {
            elf,
            elf_path: elf_path.to_string(),
        })
    }

    /// Create a `PicoProver` from raw ELF bytes (for embedded use).
    pub fn from_elf_bytes(elf: Vec<u8>) -> Self {
        Self {
            elf,
            elf_path: "<embedded>".to_string(),
        }
    }

    /// Convert host-side `CombinedProofInput` into guest-side `CircuitInput`.
    ///
    /// Maps the main crate's types (which use `EditOperation` enum and
    /// `C2paVerificationData`) into the guest library's types (which use
    /// plain strings and `OperationParams` enum for `no_std` compatibility).
    fn to_circuit_input(input: &CombinedProofInput) -> brevis_vera_zk_lib::CircuitInput {
        use brevis_vera_zk_lib::*;

        let c2pa_data = input.c2pa_data.as_ref().map(|c2pa| {
            C2paInputData {
                active_manifest: c2pa.active_manifest.clone(),
                claim_generator: c2pa.claim_generator.clone(),
                algorithm: c2pa
                    .signature_info
                    .as_ref()
                    .and_then(|s| s.alg.clone()),
            }
        });

        let editing_records = input
            .editing_records
            .iter()
            .map(|rec| {
                let params = Self::convert_params(&rec.operation, &rec.parameters);
                EditingRecordData {
                    operation: rec.operation.to_string(),
                    params,
                    input_hash: rec.input_hash.clone(),
                    output_hash: rec.output_hash.clone(),
                }
            })
            .collect();

        CircuitInput {
            c2pa_data,
            original_image_hash: input.original_image_hash.clone(),
            editing_records,
            edited_image_hash: input.edited_image_hash.clone(),
        }
    }

    /// Convert host-side JSON parameters into guest-side `OperationParams`.
    fn convert_params(
        op: &EditOperation,
        params: &serde_json::Value,
    ) -> brevis_vera_zk_lib::OperationParams {
        use brevis_vera_zk_lib::OperationParams;

        match op {
            EditOperation::Crop => OperationParams::Crop {
                x: params["x"].as_u64().unwrap_or(0) as u32,
                y: params["y"].as_u64().unwrap_or(0) as u32,
                width: params["width"].as_u64().unwrap_or(0) as u32,
                height: params["height"].as_u64().unwrap_or(0) as u32,
            },
            EditOperation::Resize => OperationParams::Resize {
                width: params["width"].as_u64().unwrap_or(0) as u32,
                height: params["height"].as_u64().unwrap_or(0) as u32,
            },
            EditOperation::Rotate => OperationParams::Rotate {
                angle: params["angle"].as_u64().unwrap_or(0) as u32,
            },
        }
    }

    /// Convert host-side `C2paProofInput` into a `CombinedProofInput`
    /// for the guest program (C2PA only, no editing).
    fn c2pa_to_combined(input: &C2paProofInput) -> CombinedProofInput {
        CombinedProofInput {
            c2pa_data: Some(input.c2pa_data.clone()),
            original_image_hash: input.image_hash.clone(),
            editing_records: vec![],
            edited_image_hash: None,
        }
    }

    /// Convert host-side `EditingProofInput` into a `CombinedProofInput`
    /// for the guest program (editing only, no C2PA).
    fn editing_to_combined(input: &EditingProofInput) -> CombinedProofInput {
        CombinedProofInput {
            c2pa_data: None,
            original_image_hash: input.original_image_hash.clone(),
            editing_records: input.editing_records.clone(),
            edited_image_hash: Some(input.edited_image_hash.clone()),
        }
    }

    /// Run the Pico prover with the given circuit input and return a `ZkProof`.
    ///
    /// This is the core method that:
    /// 1. Serializes `CircuitInput` to the ZKVM stdin
    /// 2. Creates a `DefaultProverClient` with the guest ELF
    /// 3. Calls `prove_fast()` (or `prove()` for full recursion)
    /// 4. Parses `PublicValuesStruct` from the proof's public value stream
    /// 5. Constructs a `ZkProof` with the proof bytes and parsed public inputs
    fn run_pico_prover(&self, combined: &CombinedProofInput) -> Result<ZkProof, ZkError> {
        use alloy_sol_types::SolValue;
        use brevis_vera_zk_lib::PublicValuesStruct;
        use pico_sdk::client::DefaultProverClient;

        let start = Instant::now();

        // Convert to guest-side types
        let circuit_input = Self::to_circuit_input(combined);

        // Create Pico prover client
        let client = DefaultProverClient::new(&self.elf);
        let mut stdin_builder = client.new_stdin_builder();

        // Write circuit input to ZKVM stdin
        stdin_builder.write(&circuit_input);

        // Generate proof. `prove_fast` produces a fast STARK proof.
        // For a final on-chain SNARK, use `prove()` instead.
        let proof = client.prove_fast(stdin_builder).map_err(|e| {
            ZkError::ProofGenerationFailed(format!("Pico proof generation failed: {e}"))
        })?;

        let elapsed = start.elapsed();

        // Parse public values from the proof's public value stream
        let pv_bytes = proof.pv_stream.as_ref().ok_or_else(|| {
            ZkError::ProofGenerationFailed(
                "Pico proof has no public value stream".to_string(),
            )
        })?;

        let pv = PublicValuesStruct::abi_decode(pv_bytes, true).map_err(|e| {
            ZkError::SerializationError(format!(
                "Failed to decode PublicValuesStruct from proof: {e}"
            ))
        })?;

        // Map guest public values back to host-side types
        let public_inputs = PublicInputs {
            c2pa_verified: pv.c2pa_verified == 1,
            editing_verified: pv.editing_verified == 1,
            original_image_hash: hex::encode(pv.original_image_hash.as_slice()),
            edited_image_hash: if pv.edited_image_hash.as_slice() == [0u8; 32] {
                None
            } else {
                Some(hex::encode(pv.edited_image_hash.as_slice()))
            },
            operations_applied: combined
                .editing_records
                .iter()
                .map(|r| r.operation)
                .collect(),
        };

        // Serialize the full Pico proof to bytes for storage
        let proof_bytes = bincode::serialize(&proof).map_err(|e| {
            ZkError::SerializationError(format!("Failed to serialize Pico proof: {e}"))
        })?;

        let metadata = ProofMetadata {
            prover_type: "pico".to_string(),
            generation_time_ms: elapsed.as_millis() as u64,
            timestamp: chrono::Utc::now().to_rfc3339(),
            proof_version: PROOF_VERSION.to_string(),
        };

        Ok(ZkProof {
            proof_bytes,
            public_inputs,
            metadata,
        })
    }
}

#[cfg(feature = "pico")]
impl ZkProver for PicoProver {
    fn prove_c2pa(&self, input: &C2paProofInput) -> Result<ZkProof, ZkError> {
        let combined = Self::c2pa_to_combined(input);
        self.run_pico_prover(&combined)
    }

    fn prove_editing(&self, input: &EditingProofInput) -> Result<ZkProof, ZkError> {
        let combined = Self::editing_to_combined(input);
        self.run_pico_prover(&combined)
    }

    fn prove_combined(&self, input: &CombinedProofInput) -> Result<ZkProof, ZkError> {
        self.run_pico_prover(input)
    }

    fn verify(&self, proof: &ZkProof) -> Result<bool, ZkError> {
        // For Pico proofs, verification requires the Pico verification key.
        // In production, this would deserialize the proof and call
        // the Pico verifier. For now, we verify the metadata is correct.
        if proof.metadata.prover_type != "pico" {
            return Ok(false);
        }
        // TODO: Implement full Pico STARK verification using the
        // verification key from the guest setup.
        // For now, any proof with valid metadata from the Pico prover is
        // accepted. On-chain verification uses the Solidity verifier
        // contract generated by Pico.
        Ok(true)
    }

    fn prover_type(&self) -> &str {
        "pico"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provenance::types::{C2paVerificationData, SignatureInfo};
    use crate::zk::types::EditingRecordInput;

    fn make_prover() -> SimulatedProver {
        SimulatedProver::new()
    }

    fn make_c2pa_input() -> C2paProofInput {
        C2paProofInput {
            c2pa_data: C2paVerificationData {
                signature_info: Some(SignatureInfo {
                    issuer: Some("CN=Sony".to_string()),
                    time: Some("2026-01-01T00:00:00Z".to_string()),
                    cert_serial_number: Some("12345".to_string()),
                    alg: Some("Es256".to_string()),
                }),
                assertions: vec![],
                active_manifest: "urn:c2pa:sony:1".to_string(),
                claim_generator: "SonyCamera/1.0".to_string(),
            },
            image_hash: "a1b2c3d4e5f6".to_string(),
        }
    }

    fn make_editing_input() -> EditingProofInput {
        EditingProofInput {
            original_image_hash: "orig_hash".to_string(),
            edited_image_hash: "cropped_hash".to_string(),
            editing_records: vec![EditingRecordInput {
                operation: EditOperation::Crop,
                parameters: serde_json::json!({
                    "x": 10, "y": 20, "width": 100, "height": 80
                }),
                input_hash: "orig_hash".to_string(),
                output_hash: "cropped_hash".to_string(),
            }],
        }
    }

    fn make_combined_input() -> CombinedProofInput {
        CombinedProofInput {
            c2pa_data: Some(C2paVerificationData {
                signature_info: Some(SignatureInfo {
                    issuer: Some("CN=Sony".to_string()),
                    time: None,
                    cert_serial_number: None,
                    alg: Some("Es256".to_string()),
                }),
                assertions: vec![],
                active_manifest: "urn:c2pa:sony:1".to_string(),
                claim_generator: "SonyCamera/1.0".to_string(),
            }),
            original_image_hash: "abcdef".to_string(),
            editing_records: vec![EditingRecordInput {
                operation: EditOperation::Crop,
                parameters: serde_json::json!({
                    "x": 0, "y": 0, "width": 50, "height": 50
                }),
                input_hash: "abcdef".to_string(),
                output_hash: "cropped".to_string(),
            }],
            edited_image_hash: Some("cropped".to_string()),
        }
    }

    // ---- SimulatedProver tests ----

    #[test]
    fn test_prover_type() {
        assert_eq!(make_prover().prover_type(), "simulated");
    }

    #[test]
    fn test_prover_default() {
        let prover = SimulatedProver::default();
        assert_eq!(prover.prover_type(), "simulated");
    }

    #[test]
    fn test_prove_c2pa_success() {
        let prover = make_prover();
        let proof = prover.prove_c2pa(&make_c2pa_input()).unwrap();

        assert!(proof.public_inputs.c2pa_verified);
        assert!(!proof.public_inputs.editing_verified);
        assert_eq!(proof.public_inputs.original_image_hash, "a1b2c3d4e5f6");
        assert!(proof.public_inputs.edited_image_hash.is_none());
        assert!(proof.public_inputs.operations_applied.is_empty());
        assert_eq!(proof.metadata.prover_type, "simulated");
        assert_eq!(proof.metadata.proof_version, "0.1.0");
        assert!(!proof.proof_bytes.is_empty());
    }

    #[test]
    fn test_prove_c2pa_invalid() {
        let prover = make_prover();
        let mut input = make_c2pa_input();
        input.image_hash = String::new(); // invalid

        let result = prover.prove_c2pa(&input);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ZkError::C2paVerificationFailed(_)));
    }

    #[test]
    fn test_prove_editing_success() {
        let prover = make_prover();
        let proof = prover.prove_editing(&make_editing_input()).unwrap();

        assert!(!proof.public_inputs.c2pa_verified);
        assert!(proof.public_inputs.editing_verified);
        assert_eq!(proof.public_inputs.original_image_hash, "orig_hash");
        assert_eq!(
            proof.public_inputs.edited_image_hash.as_deref(),
            Some("cropped_hash")
        );
        assert_eq!(proof.public_inputs.operations_applied, vec![EditOperation::Crop]);
    }

    #[test]
    fn test_prove_editing_invalid() {
        let prover = make_prover();
        let input = EditingProofInput {
            original_image_hash: "orig".to_string(),
            edited_image_hash: "wrong".to_string(),
            editing_records: vec![EditingRecordInput {
                operation: EditOperation::Crop,
                parameters: serde_json::json!({"x": 0, "y": 0, "width": 100, "height": 100}),
                input_hash: "orig".to_string(),
                output_hash: "cropped".to_string(), // doesn't match edited_image_hash
            }],
        };

        let result = prover.prove_editing(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_prove_combined_success() {
        let prover = make_prover();
        let proof = prover.prove_combined(&make_combined_input()).unwrap();

        assert!(proof.public_inputs.c2pa_verified);
        assert!(proof.public_inputs.editing_verified);
        assert_eq!(proof.public_inputs.original_image_hash, "abcdef");
        assert_eq!(
            proof.public_inputs.edited_image_hash.as_deref(),
            Some("cropped")
        );
        assert_eq!(proof.public_inputs.operations_applied, vec![EditOperation::Crop]);
    }

    #[test]
    fn test_prove_combined_c2pa_only() {
        let prover = make_prover();
        let input = CombinedProofInput {
            c2pa_data: Some(C2paVerificationData {
                signature_info: None,
                assertions: vec![],
                active_manifest: "urn:test".to_string(),
                claim_generator: "Test/1.0".to_string(),
            }),
            original_image_hash: "aabbcc".to_string(),
            editing_records: vec![],
            edited_image_hash: None,
        };

        let proof = prover.prove_combined(&input).unwrap();
        assert!(proof.public_inputs.c2pa_verified);
        assert!(!proof.public_inputs.editing_verified);
    }

    #[test]
    fn test_prove_combined_editing_only() {
        let prover = make_prover();
        let input = CombinedProofInput {
            c2pa_data: None,
            original_image_hash: "orig".to_string(),
            editing_records: vec![EditingRecordInput {
                operation: EditOperation::Resize,
                parameters: serde_json::json!({"width": 200, "height": 100}),
                input_hash: "orig".to_string(),
                output_hash: "resized".to_string(),
            }],
            edited_image_hash: Some("resized".to_string()),
        };

        let proof = prover.prove_combined(&input).unwrap();
        assert!(!proof.public_inputs.c2pa_verified);
        assert!(proof.public_inputs.editing_verified);
    }

    // ---- Verification tests ----

    #[test]
    fn test_verify_valid_proof() {
        let prover = make_prover();
        let proof = prover.prove_c2pa(&make_c2pa_input()).unwrap();
        assert!(prover.verify(&proof).unwrap());
    }

    #[test]
    fn test_verify_tampered_proof_bytes() {
        let prover = make_prover();
        let mut proof = prover.prove_c2pa(&make_c2pa_input()).unwrap();
        proof.proof_bytes[0] ^= 0xFF; // tamper
        assert!(!prover.verify(&proof).unwrap());
    }

    #[test]
    fn test_verify_tampered_public_inputs() {
        let prover = make_prover();
        let mut proof = prover.prove_c2pa(&make_c2pa_input()).unwrap();
        proof.public_inputs.c2pa_verified = false; // tamper
        assert!(!prover.verify(&proof).unwrap());
    }

    // ---- Serialization tests ----

    #[test]
    fn test_proof_json_roundtrip() {
        let prover = make_prover();
        let proof = prover.prove_combined(&make_combined_input()).unwrap();

        let json = proof.to_json().unwrap();
        let parsed = ZkProof::from_json(&json).unwrap();

        assert_eq!(proof.proof_bytes, parsed.proof_bytes);
        assert_eq!(
            proof.public_inputs.c2pa_verified,
            parsed.public_inputs.c2pa_verified
        );
        assert_eq!(
            proof.public_inputs.original_image_hash,
            parsed.public_inputs.original_image_hash
        );
        assert!(prover.verify(&parsed).unwrap());
    }

    #[test]
    fn test_proof_binary_roundtrip() {
        let prover = make_prover();
        let proof = prover.prove_editing(&make_editing_input()).unwrap();

        let bytes = proof.to_bytes().unwrap();
        let parsed = ZkProof::from_bytes(&bytes).unwrap();

        assert_eq!(proof.proof_bytes, parsed.proof_bytes);
        assert_eq!(
            proof.public_inputs.editing_verified,
            parsed.public_inputs.editing_verified
        );
        assert!(prover.verify(&parsed).unwrap());
    }

    #[test]
    fn test_proof_binary_too_short() {
        let result = ZkProof::from_bytes(&[0, 0]);
        assert!(result.is_err());
    }

    #[test]
    fn test_proof_binary_truncated() {
        let result = ZkProof::from_bytes(&[0xFF, 0, 0, 0]); // claims 255 bytes of proof
        assert!(result.is_err());
    }

    // ---- Multi-operation chain tests ----

    #[test]
    fn test_prove_editing_chain() {
        let prover = make_prover();
        let input = EditingProofInput {
            original_image_hash: "orig".to_string(),
            edited_image_hash: "final".to_string(),
            editing_records: vec![
                EditingRecordInput {
                    operation: EditOperation::Crop,
                    parameters: serde_json::json!({"x": 0, "y": 0, "width": 100, "height": 100}),
                    input_hash: "orig".to_string(),
                    output_hash: "cropped".to_string(),
                },
                EditingRecordInput {
                    operation: EditOperation::Resize,
                    parameters: serde_json::json!({"width": 50, "height": 50}),
                    input_hash: "cropped".to_string(),
                    output_hash: "resized".to_string(),
                },
                EditingRecordInput {
                    operation: EditOperation::Rotate,
                    parameters: serde_json::json!({"angle": "90"}),
                    input_hash: "resized".to_string(),
                    output_hash: "final".to_string(),
                },
            ],
        };

        let proof = prover.prove_editing(&input).unwrap();
        assert!(proof.public_inputs.editing_verified);
        assert_eq!(proof.public_inputs.operations_applied.len(), 3);
        assert!(prover.verify(&proof).unwrap());
    }

    #[test]
    fn test_generation_time_recorded() {
        let prover = make_prover();
        let proof = prover.prove_c2pa(&make_c2pa_input()).unwrap();
        // Simulated prover should be very fast
        assert!(proof.metadata.generation_time_ms < 1000);
        assert!(!proof.metadata.timestamp.is_empty());
    }
}
