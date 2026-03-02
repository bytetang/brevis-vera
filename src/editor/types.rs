//! Data types for the Editing Layer.
//!
//! Defines core data structures: editing operations, parameters,
//! editing records, and error types.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

// ---------------------------------------------------------------------------
// Editing operations
// ---------------------------------------------------------------------------

/// Supported editing operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EditOperation {
    /// Crop: extract a rectangular region from the image
    Crop,
    /// Resize: scale the image to new dimensions
    Resize,
    /// Rotate: rotate the image by 90, 180, or 270 degrees
    Rotate,
}

impl std::fmt::Display for EditOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditOperation::Crop => write!(f, "crop"),
            EditOperation::Resize => write!(f, "resize"),
            EditOperation::Rotate => write!(f, "rotate"),
        }
    }
}

/// Parameters for a crop operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CropParams {
    /// X coordinate of the top-left corner
    pub x: u32,
    /// Y coordinate of the top-left corner
    pub y: u32,
    /// Width of the crop region
    pub width: u32,
    /// Height of the crop region
    pub height: u32,
}

/// Parameters for a resize operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResizeParams {
    /// Target width
    pub width: u32,
    /// Target height
    pub height: u32,
}

/// Supported rotation angles in degrees (clockwise).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RotationAngle {
    /// 90 degrees clockwise
    #[serde(rename = "90")]
    Deg90,
    /// 180 degrees
    #[serde(rename = "180")]
    Deg180,
    /// 270 degrees clockwise (90 counter-clockwise)
    #[serde(rename = "270")]
    Deg270,
}

impl std::fmt::Display for RotationAngle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RotationAngle::Deg90 => write!(f, "90"),
            RotationAngle::Deg180 => write!(f, "180"),
            RotationAngle::Deg270 => write!(f, "270"),
        }
    }
}

/// Parameters for a rotate operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RotateParams {
    /// Rotation angle (90, 180, or 270 degrees)
    pub angle: RotationAngle,
}

/// Union of all editing parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditParams {
    /// Crop parameters
    Crop(CropParams),
    /// Resize parameters
    Resize(ResizeParams),
    /// Rotate parameters
    Rotate(RotateParams),
}

// ---------------------------------------------------------------------------
// Editing record
// ---------------------------------------------------------------------------

/// Record of an editing operation performed on an image.
///
/// This is returned after each edit and passed to the ZK Proof Layer
/// to prove which operations were applied without revealing specifics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditingRecord {
    /// Type of operation performed
    pub operation: EditOperation,
    /// Operation-specific parameters
    pub parameters: serde_json::Value,
    /// SHA-256 hash of the original image (before edit)
    pub original_image_hash: String,
    /// SHA-256 hash of the edited image (after edit)
    pub edited_image_hash: String,
    /// Timestamp of the operation (ISO 8601)
    pub timestamp: String,
}

/// Result of an editing operation.
///
/// Contains the edited image bytes, the editing record for ZK proof,
/// and dimensional information.
#[derive(Debug, Clone)]
pub struct EditResult {
    /// Edited image bytes (PNG-encoded)
    pub image_bytes: Vec<u8>,
    /// Editing record for ZK proof generation
    pub record: EditingRecord,
    /// Width of the edited image
    pub width: u32,
    /// Height of the edited image
    pub height: u32,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Compute SHA-256 hash of data and return as hex string.
pub fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Errors that can occur during editing operations.
#[derive(Error, Debug)]
pub enum EditorError {
    /// The input image could not be decoded.
    #[error("Failed to decode image: {0}")]
    DecodeError(String),

    /// The crop region is outside image bounds.
    #[error("Crop region out of bounds: image is {img_w}x{img_h}, crop at ({x},{y}) size {w}x{h}")]
    CropOutOfBounds {
        img_w: u32,
        img_h: u32,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
    },

    /// Resize dimensions are invalid (zero width or height).
    #[error("Invalid resize dimensions: {width}x{height}")]
    InvalidResizeDimensions { width: u32, height: u32 },

    /// Image encoding failed.
    #[error("Failed to encode image: {0}")]
    EncodeError(String),

    /// An I/O error occurred.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
