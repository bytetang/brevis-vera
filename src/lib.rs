//! Brevis Vera - End-to-end digital media authenticity attestation system.
//!
//! Brevis Vera bridges hardware-level provenance (C2PA signatures) and
//! software-level editing verification using Zero-Knowledge Proofs.
//!
//! # Architecture
//!
//! The system consists of four layers:
//! 1. **Capture & Provenance Layer** - Extracts C2PA metadata from media files
//! 2. **Editing Layer** - Applies image transformations (crop, resize, etc.)
//! 3. **ZK Proof Layer** - Generates zero-knowledge proofs in ZKVM
//! 4. **Verification Layer** - Verifies proofs and reports authenticity
//!
//! # Current Module
//!
//! The `provenance` module implements the Capture & Provenance Layer,
//! extracting C2PA metadata and preparing data for the ZK Proof Layer.
//!
//! # Example
//!
//! ```no_run
//! use brevis_vera::provenance::ProvenanceProcessor;
//! use std::path::Path;
//!
//! let processor = ProvenanceProcessor::new();
//! let result = processor.process(Path::new("image.jpg")).unwrap();
//!
//! if let Some(ref metadata) = result.c2pa_metadata {
//!     println!("C2PA found: {}", metadata.claim_generator);
//! }
//! if let Some(ref zkvm_input) = result.zkvm_input {
//!     println!("Image hash: {}", zkvm_input.image_hash);
//! }
//! ```

pub mod provenance;
