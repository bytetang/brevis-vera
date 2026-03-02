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
//! # Modules
//!
//! - [`provenance`] — Capture & Provenance Layer: C2PA metadata extraction
//! - [`editor`] — Editing Layer: image transformations and REST API
//! - [`zk`] — ZK Proof Layer: zero-knowledge proof generation and verification
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

pub mod editor;
pub mod provenance;
pub mod zk;
