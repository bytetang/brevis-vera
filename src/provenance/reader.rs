//! Media file reader with format detection.
//!
//! Reads JPEG and PNG files from disk and detects their format
//! based on magic bytes (file signatures).
//!
//! # Supported Formats
//!
//! | Format | Magic Bytes |
//! |--------|-------------|
//! | JPEG   | `FF D8 FF`  |
//! | PNG    | `89 50 4E 47` |

use std::fs;
use std::path::Path;

use super::types::{MediaFile, MediaFormat, ProvenanceError};

/// JPEG magic bytes: FF D8 FF
const JPEG_MAGIC: [u8; 3] = [0xFF, 0xD8, 0xFF];

/// PNG magic bytes: 89 50 4E 47
const PNG_MAGIC: [u8; 4] = [0x89, 0x50, 0x4E, 0x47];

/// Minimum file size to detect format (need at least 4 bytes for PNG magic).
const MIN_FILE_SIZE: usize = 4;

/// Read a media file from disk and detect its format.
///
/// # Arguments
/// * `path` - Path to the media file
///
/// # Returns
/// A [`MediaFile`] with the raw bytes, detected format, and original path.
///
/// # Errors
/// * [`ProvenanceError::Io`] if the file cannot be read
/// * [`ProvenanceError::UnsupportedFormat`] if the file is not JPEG or PNG
pub fn read_media(path: &Path) -> Result<MediaFile, ProvenanceError> {
    let bytes = fs::read(path)?;
    let format = detect_format(&bytes)?;

    Ok(MediaFile {
        bytes,
        format,
        path: path.to_path_buf(),
    })
}

/// Detect the media format from raw file bytes using magic byte signatures.
///
/// # Arguments
/// * `bytes` - Raw file bytes (at least 4 bytes required)
///
/// # Returns
/// * [`MediaFormat::Jpeg`] if JPEG magic bytes are detected
/// * [`MediaFormat::Png`] if PNG magic bytes are detected
///
/// # Errors
/// * [`ProvenanceError::UnsupportedFormat`] if the format is not recognized
pub fn detect_format(bytes: &[u8]) -> Result<MediaFormat, ProvenanceError> {
    if bytes.len() < MIN_FILE_SIZE {
        return Err(ProvenanceError::UnsupportedFormat);
    }

    if bytes[..3] == JPEG_MAGIC {
        return Ok(MediaFormat::Jpeg);
    }

    if bytes[..4] == PNG_MAGIC {
        return Ok(MediaFormat::Png);
    }

    Err(ProvenanceError::UnsupportedFormat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_jpeg() {
        let bytes = vec![0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10];
        assert_eq!(detect_format(&bytes).unwrap(), MediaFormat::Jpeg);
    }

    #[test]
    fn test_detect_jpeg_exif() {
        // JPEG with Exif APP1 marker
        let bytes = vec![0xFF, 0xD8, 0xFF, 0xE1, 0x00, 0x10];
        assert_eq!(detect_format(&bytes).unwrap(), MediaFormat::Jpeg);
    }

    #[test]
    fn test_detect_png() {
        let bytes = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        assert_eq!(detect_format(&bytes).unwrap(), MediaFormat::Png);
    }

    #[test]
    fn test_detect_unsupported_gif() {
        let bytes = b"GIF89a".to_vec();
        assert!(matches!(
            detect_format(&bytes),
            Err(ProvenanceError::UnsupportedFormat)
        ));
    }

    #[test]
    fn test_detect_unsupported_random() {
        let bytes = vec![0x00, 0x01, 0x02, 0x03];
        assert!(matches!(
            detect_format(&bytes),
            Err(ProvenanceError::UnsupportedFormat)
        ));
    }

    #[test]
    fn test_detect_too_short() {
        let bytes = vec![0xFF, 0xD8];
        assert!(matches!(
            detect_format(&bytes),
            Err(ProvenanceError::UnsupportedFormat)
        ));
    }

    #[test]
    fn test_detect_empty() {
        let bytes: Vec<u8> = vec![];
        assert!(matches!(
            detect_format(&bytes),
            Err(ProvenanceError::UnsupportedFormat)
        ));
    }

    #[test]
    fn test_read_nonexistent_file() {
        let result = read_media(Path::new("/nonexistent/file.jpg"));
        assert!(matches!(result, Err(ProvenanceError::Io(_))));
    }
}
