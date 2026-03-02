//! ZK Proof Generation Layer for Brevis Vera.
//!
//! This module generates zero-knowledge proofs that verify both:
//! 1. **C2PA Provenance** — the original image has valid C2PA metadata
//! 2. **Editing Proof** — the claimed edits (crop, resize, rotate) were applied correctly
//!
//! # Architecture
//!
//! ```text
//! ┌──────────────────────────────────────────────────┐
//! │              ZK Proof Generation Layer            │
//! │                                                  │
//! │  ┌────────────┐  ┌─────────────┐  ┌──────────┐  │
//! │  │  Circuits   │  │   Prover    │  │  Types   │  │
//! │  │ ─────────── │  │ ─────────── │  │          │  │
//! │  │ C2PA verify │  │ Simulated   │  │ Inputs   │  │
//! │  │ Crop verify │  │ (test/dev)  │  │ Outputs  │  │
//! │  │ Resize      │  │             │  │ Proofs   │  │
//! │  │ Combined    │  │ Pico ZKVM   │  │ Errors   │  │
//! │  │             │  │ (production)│  │          │  │
//! │  └────────────┘  └─────────────┘  └──────────┘  │
//! └──────────────────────────────────────────────────┘
//! ```
//!
//! # Prover Backends
//!
//! The module provides a trait-based prover design:
//! - [`SimulatedProver`] — runs circuit logic natively for development/testing
//! - [`PicoProver`] — runs circuits inside Pico ZKVM for real STARK/SNARK proofs
//!   (requires the `pico` feature flag)
//!
//! # Usage
//!
//! ```no_run
//! use brevis_vera::zk::{ZkProver, SimulatedProver, CombinedProofInput};
//! use brevis_vera::provenance::ProvenanceProcessor;
//! use std::path::Path;
//!
//! // 1. Extract provenance
//! let processor = ProvenanceProcessor::new();
//! let prov = processor.process(Path::new("image.jpg")).unwrap();
//!
//! // 2. Generate combined proof
//! let prover = SimulatedProver::new();
//! let input = CombinedProofInput {
//!     c2pa_data: prov.zkvm_input.map(|z| z.c2pa_data),
//!     original_image_hash: prov.image_hash.to_hex(),
//!     editing_records: vec![],
//!     edited_image_hash: None,
//! };
//! let proof = prover.prove_combined(&input).unwrap();
//! println!("Proof verified: {}", prover.verify(&proof).unwrap());
//! ```
//!
//! # References
//!
//! - [Pico ZKVM Documentation](https://pico-docs.brevis.network/)
//! - [Pico ZKVM GitHub](https://github.com/brevis-network/pico)
//! - [brevis-network/signatures](https://github.com/brevis-network/signatures) — ECDSA P-256 circuits

pub mod circuits;
pub mod prover;
pub mod types;

// Re-export primary types for convenience
pub use prover::{SimulatedProver, ZkProver};
#[cfg(feature = "pico")]
pub use prover::PicoProver;
pub use types::{
    C2paProofInput, CombinedProofInput, EditingProofInput, ProofMetadata, PublicInputs, ZkError,
    ZkProof,
};
