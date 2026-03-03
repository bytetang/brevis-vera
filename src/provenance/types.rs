//! Data types for the Capture & Provenance Layer.
//!
//! This module defines the core data structures used throughout the provenance
//! pipeline: media file representation, C2PA metadata, and ZKVM input format.
//!
//! # ZKVM Output Format
//!
//! The [`ZkvmInput`] struct is the final output of this layer, containing:
//! - C2PA verification data (signature info, claims/assertions)
//! - Image hash (SHA-256)
//!
//! This data is passed to the ZK Proof Layer for structural verification
//! inside the ZKVM circuit.  Cryptographic signature verification is
//! performed on the host by the `c2pa` library.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

// ---------------------------------------------------------------------------
// Media types
// ---------------------------------------------------------------------------

/// Supported media file formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaFormat {
    /// JPEG image format (JFIF/Exif)
    Jpeg,
    /// PNG image format
    Png,
}

impl std::fmt::Display for MediaFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MediaFormat::Jpeg => write!(f, "JPEG"),
            MediaFormat::Png => write!(f, "PNG"),
        }
    }
}

/// A media file that has been read from disk and format-detected.
#[derive(Debug, Clone)]
pub struct MediaFile {
    /// Raw file bytes
    pub bytes: Vec<u8>,
    /// Detected media format
    pub format: MediaFormat,
    /// Original file path
    pub path: PathBuf,
}

// ---------------------------------------------------------------------------
// Image hash
// ---------------------------------------------------------------------------

/// SHA-256 hash of image data.
///
/// Used to bind the image content to the C2PA manifest for ZKVM verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageHash(pub [u8; 32]);

impl ImageHash {
    /// Returns the hash as a lowercase hex string.
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
}

impl std::fmt::Display for ImageHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

// ---------------------------------------------------------------------------
// C2PA metadata types
// ---------------------------------------------------------------------------

/// Extracted C2PA metadata from a media file.
///
/// Contains all provenance information parsed from the C2PA manifest store
/// embedded in the media file. This includes signature details, claims,
/// assertions, and ingredient references.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C2paMetadata {
    /// Active manifest label (URN identifier)
    pub active_manifest: String,
    /// Claim generator identifier (software that created the C2PA manifest)
    pub claim_generator: String,
    /// Asset title
    pub title: Option<String>,
    /// Media format MIME type (e.g., "image/jpeg")
    pub format: String,
    /// Signature information (issuer, time, algorithm)
    pub signature_info: Option<SignatureInfo>,
    /// Claims/assertions from the manifest
    pub assertions: Vec<ClaimAssertion>,
    /// Ingredient references (previous versions or sources)
    pub ingredients: Vec<Ingredient>,
    /// Full manifest store JSON for detailed inspection and debugging
    pub raw_manifest_store: serde_json::Value,
}

/// C2PA signature information.
///
/// Contains details about the digital signature applied to the C2PA manifest.
/// Supports multiple C2PA signing algorithms (Es256, PS256, Ed25519, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureInfo {
    /// Certificate issuer distinguished name
    pub issuer: Option<String>,
    /// Signing timestamp (ISO 8601 format)
    pub time: Option<String>,
    /// Certificate serial number
    pub cert_serial_number: Option<String>,
    /// Signing algorithm (e.g., "Es256", "PS256", "Ed25519")
    pub alg: Option<String>,
    /// ECDSA signature (r, s components) if available
    pub ecdsa_signature: Option<EcdsaSignature>,
    /// Public key used for signature verification (hex-encoded, uncompressed 04 prefix)
    pub public_key: Option<String>,
}

/// ECDSA P-256 signature components (r, s values).
///
/// These are the raw signature values that can be verified
/// using ECDSA P-256 against a public key and message hash.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcdsaSignature {
    /// Signature r component (32 bytes)
    pub r: Vec<u8>,
    /// Signature s component (32 bytes)
    pub s: Vec<u8>,
}

impl EcdsaSignature {
    /// Returns the signature as hex-encoded strings.
    pub fn to_hex(&self) -> (String, String) {
        (hex::encode(&self.r), hex::encode(&self.s))
    }
}

/// A C2PA assertion/claim.
///
/// Assertions are the core data units in a C2PA manifest, describing
/// actions taken on the asset, authorship, and other provenance data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimAssertion {
    /// Assertion label (e.g., "c2pa.actions", "stds.schema-org.CreativeWork")
    pub label: String,
    /// Assertion data as a JSON value
    pub data: serde_json::Value,
}

/// A C2PA ingredient reference.
///
/// Ingredients represent previous versions or source assets that
/// contributed to the current asset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    /// Title of the ingredient asset
    pub title: Option<String>,
    /// Format MIME type of the ingredient
    pub format: Option<String>,
    /// Relationship to the parent manifest (e.g., "parentOf")
    pub relationship: Option<String>,
}

// ---------------------------------------------------------------------------
// ZKVM input types
// ---------------------------------------------------------------------------

/// Input data prepared for ZKVM processing.
///
/// This struct contains all the data needed by the ZK Proof Layer
/// to verify C2PA provenance inside the ZKVM circuit.
///
/// # ZKVM Verification Flow
///
/// The ZKVM circuit will:
/// 1. Verify the certificate chain validity
/// 2. Verify the ECDSA P-256 signature over the image hash
/// 3. Output: C2PA provenance verified (privacy-preserving)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkvmInput {
    /// C2PA verification data for the ZKVM circuit
    pub c2pa_data: C2paVerificationData,
    /// SHA-256 hash of the original image
    pub image_hash: ImageHash,
}

/// C2PA data needed for ZKVM verification.
///
/// This subset of C2PA metadata is passed to the ZKVM circuit for
/// privacy-preserving signature verification using ECDSA P-256.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C2paVerificationData {
    /// Signature information for ECDSA P-256 verification
    pub signature_info: Option<SignatureInfo>,
    /// Claims/assertions to verify
    pub assertions: Vec<ClaimAssertion>,
    /// Active manifest label
    pub active_manifest: String,
    /// Claim generator identifier
    pub claim_generator: String,
    /// ECDSA signature (r, s) for cryptographic verification in ZKVM
    pub ecdsa_signature: Option<EcdsaSignature>,
    /// Public key for ECDSA verification (hex-encoded, uncompressed 04 prefix)
    pub public_key: Option<String>,
}

// ---------------------------------------------------------------------------
// Provenance result
// ---------------------------------------------------------------------------

/// Complete result of provenance processing.
///
/// Contains all extracted metadata, the original image data,
/// and prepared ZKVM input (if C2PA metadata was found).
#[derive(Debug, Clone, Serialize)]
pub struct ProvenanceResult {
    /// C2PA metadata extracted from the file (`None` if no C2PA data present)
    pub c2pa_metadata: Option<C2paMetadata>,
    /// SHA-256 hash of the original image bytes
    pub image_hash: ImageHash,
    /// Original image bytes (skipped during serialization)
    #[serde(skip)]
    pub original_image: Vec<u8>,
    /// ZKVM input data (present only when C2PA metadata was found)
    pub zkvm_input: Option<ZkvmInput>,
    /// Detected media format
    pub format: MediaFormat,
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Errors that can occur during provenance processing.
#[derive(Error, Debug)]
pub enum ProvenanceError {
    /// File I/O error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// The file is not a supported media format (JPEG or PNG)
    #[error("Unsupported media format (expected JPEG or PNG)")]
    UnsupportedFormat,

    /// Error parsing C2PA metadata
    #[error("C2PA parsing error: {0}")]
    C2paError(String),

    /// No C2PA metadata found in the file
    #[error("No C2PA metadata found in file")]
    NoMetadata,

    /// The image data is invalid or corrupt
    #[error("Invalid image data: {0}")]
    InvalidImage(String),

    /// Failed to extract signature data from manifest
    #[error("Failed to extract signature: {0}")]
    SignatureExtractionError(String),
}
