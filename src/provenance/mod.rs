//! Capture & Provenance Layer
//!
//! This module extracts C2PA metadata from media files and prepares data
//! for the ZK Proof Layer. C2PA signature verification (using ECDSA P-256)
//! happens inside the ZKVM circuit — this layer only extracts and structures
//! the provenance data.
//!
//! # Architecture
//!
//! ```text
//! Media File ─→ Reader ─→ C2PA Parser ─→ Extractor ─→ ZKVM Input
//!                │                           │
//!                └─ format detection         └─ SHA-256 image hash
//! ```
//!
//! # Submodules
//!
//! - [`reader`] — Media file reading and format detection (JPEG, PNG)
//! - [`parser`] — C2PA metadata extraction using the `c2pa` crate
//! - [`extractor`] — Image hashing and ZKVM input construction
//! - [`types`] — Data structures for metadata, errors, and ZKVM input
//!
//! # Usage
//!
//! ```no_run
//! use brevis_vera::provenance::ProvenanceProcessor;
//! use std::path::Path;
//!
//! let processor = ProvenanceProcessor::new();
//! let result = processor.process(Path::new("signed_image.jpg")).unwrap();
//!
//! if let Some(ref metadata) = result.c2pa_metadata {
//!     println!("Claim generator: {}", metadata.claim_generator);
//!     if let Some(ref sig) = metadata.signature_info {
//!         println!("Signed by: {:?}", sig.issuer);
//!         println!("Algorithm: {:?}", sig.alg);
//!     }
//! }
//!
//! if let Some(ref zkvm_input) = result.zkvm_input {
//!     println!("Image hash for ZKVM: {}", zkvm_input.image_hash);
//! }
//! ```

pub mod extractor;
pub mod parser;
pub mod reader;
pub mod types;

pub use types::*;

use std::path::Path;

/// Main entry point for the Capture & Provenance Layer.
///
/// Orchestrates the full provenance extraction pipeline:
/// 1. Read and validate the media file format
/// 2. Parse C2PA metadata (if present)
/// 3. Compute the SHA-256 image hash
/// 4. Build the ZKVM input structure
pub struct ProvenanceProcessor;

impl ProvenanceProcessor {
    /// Creates a new `ProvenanceProcessor`.
    pub fn new() -> Self {
        Self
    }

    /// Process a media file to extract C2PA metadata and prepare ZKVM input.
    ///
    /// # Arguments
    /// * `path` - Path to a JPEG or PNG media file
    ///
    /// # Returns
    /// A [`ProvenanceResult`] containing:
    /// - Extracted C2PA metadata (if present)
    /// - SHA-256 hash of the image file
    /// - Original image bytes
    /// - ZKVM input data (if C2PA metadata was found)
    /// - Detected media format
    ///
    /// # Errors
    /// * [`ProvenanceError::UnsupportedFormat`] — file is not JPEG or PNG
    /// * [`ProvenanceError::Io`] — file cannot be read
    /// * [`ProvenanceError::C2paError`] — C2PA parsing fails
    pub fn process(&self, path: &Path) -> Result<ProvenanceResult, ProvenanceError> {
        // Step 1: Read and validate media file format
        let media_file = reader::read_media(path)?;

        // Step 2: Parse C2PA metadata (returns None if not present)
        let c2pa_metadata = parser::parse_c2pa(path)?;

        // Step 3: Compute SHA-256 hash of the image bytes
        let image_hash = extractor::compute_image_hash(&media_file.bytes);

        // Step 4: Build ZKVM input if C2PA metadata was found
        let zkvm_input = c2pa_metadata
            .as_ref()
            .map(|metadata| extractor::build_zkvm_input(metadata, image_hash));

        Ok(ProvenanceResult {
            c2pa_metadata,
            image_hash,
            original_image: media_file.bytes,
            zkvm_input,
            format: media_file.format,
        })
    }
}

impl Default for ProvenanceProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_image_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("imgs/test_img.JPG")
    }

    #[test]
    fn test_process_test_image() {
        let path = test_image_path();
        if !path.exists() {
            eprintln!("Skipping: test image not found at {:?}", path);
            return;
        }

        let processor = ProvenanceProcessor::new();
        let result = processor.process(&path).unwrap();

        // Should detect JPEG format
        assert_eq!(result.format, MediaFormat::Jpeg);

        // Image bytes should be non-empty
        assert!(!result.original_image.is_empty());

        // Hash should be non-zero
        assert_ne!(result.image_hash.0, [0u8; 32]);

        // Hash should be deterministic
        let result2 = processor.process(&path).unwrap();
        assert_eq!(result.image_hash, result2.image_hash);
    }

    #[test]
    fn test_process_nonexistent() {
        let processor = ProvenanceProcessor::new();
        let result = processor.process(Path::new("/does/not/exist.jpg"));
        assert!(result.is_err());
    }

    #[test]
    fn test_processor_default() {
        let _processor = ProvenanceProcessor::default();
    }
}
