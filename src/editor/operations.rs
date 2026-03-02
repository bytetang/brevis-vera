//! Core image editing operations using the Photon library.
//!
//! Supports crop, resize, and rotation operations. Each function takes
//! raw image bytes (JPEG or PNG) and returns the processed image as PNG
//! bytes, along with an [`EditingRecord`] for the ZK Proof Layer.
//!
//! # Photon Integration
//!
//! Photon provides high-performance image processing. This module uses
//! Photon for crop and the `image` crate for resize and rotation, since
//! Photon's resize/rotate API is limited.

use chrono::Utc;
use image::imageops::FilterType;
use image::DynamicImage;
use photon_rs::transform::crop as photon_crop;
use photon_rs::PhotonImage;

use super::types::{
    CropParams, EditOperation, EditResult, EditingRecord, EditorError, ResizeParams, RotateParams,
    RotationAngle, sha256_hex,
};

/// Crop an image to the specified rectangular region.
///
/// # Arguments
/// * `image_bytes` - Raw image bytes (JPEG or PNG)
/// * `params` - Crop coordinates and dimensions
///
/// # Errors
/// * [`EditorError::DecodeError`] if the image cannot be decoded
/// * [`EditorError::CropOutOfBounds`] if the crop region exceeds image bounds
pub fn crop(image_bytes: &[u8], params: &CropParams) -> Result<EditResult, EditorError> {
    let original_hash = sha256_hex(image_bytes);

    let mut img = PhotonImage::new_from_byteslice(image_bytes.to_vec());

    let img_w = img.get_width();
    let img_h = img.get_height();

    // Validate crop bounds
    if params.x + params.width > img_w || params.y + params.height > img_h {
        return Err(EditorError::CropOutOfBounds {
            img_w,
            img_h,
            x: params.x,
            y: params.y,
            w: params.width,
            h: params.height,
        });
    }

    if params.width == 0 || params.height == 0 {
        return Err(EditorError::CropOutOfBounds {
            img_w,
            img_h,
            x: params.x,
            y: params.y,
            w: params.width,
            h: params.height,
        });
    }

    let cropped = photon_crop(
        &mut img,
        params.x,
        params.y,
        params.x + params.width,
        params.y + params.height,
    );

    let out_bytes = cropped.get_bytes();
    let edited_hash = sha256_hex(&out_bytes);

    Ok(EditResult {
        width: cropped.get_width(),
        height: cropped.get_height(),
        image_bytes: out_bytes,
        record: EditingRecord {
            operation: EditOperation::Crop,
            parameters: serde_json::json!({
                "x": params.x,
                "y": params.y,
                "width": params.width,
                "height": params.height,
            }),
            original_image_hash: original_hash,
            edited_image_hash: edited_hash,
            timestamp: Utc::now().to_rfc3339(),
        },
    })
}

/// Resize an image to the specified dimensions.
///
/// Uses Lanczos3 filter for high-quality downsampling.
///
/// # Arguments
/// * `image_bytes` - Raw image bytes (JPEG or PNG)
/// * `params` - Target width and height
///
/// # Errors
/// * [`EditorError::DecodeError`] if the image cannot be decoded
/// * [`EditorError::InvalidResizeDimensions`] if width or height is zero
pub fn resize(image_bytes: &[u8], params: &ResizeParams) -> Result<EditResult, EditorError> {
    let original_hash = sha256_hex(image_bytes);

    if params.width == 0 || params.height == 0 {
        return Err(EditorError::InvalidResizeDimensions {
            width: params.width,
            height: params.height,
        });
    }

    let img = load_dynamic_image(image_bytes)?;
    let original_w = img.width();
    let original_h = img.height();

    let resized = img.resize_exact(params.width, params.height, FilterType::Lanczos3);

    let out_bytes = encode_png(&resized)?;
    let edited_hash = sha256_hex(&out_bytes);

    Ok(EditResult {
        width: resized.width(),
        height: resized.height(),
        image_bytes: out_bytes,
        record: EditingRecord {
            operation: EditOperation::Resize,
            parameters: serde_json::json!({
                "original_width": original_w,
                "original_height": original_h,
                "new_width": params.width,
                "new_height": params.height,
            }),
            original_image_hash: original_hash,
            edited_image_hash: edited_hash,
            timestamp: Utc::now().to_rfc3339(),
        },
    })
}

/// Rotate an image by 90, 180, or 270 degrees clockwise.
///
/// # Arguments
/// * `image_bytes` - Raw image bytes (JPEG or PNG)
/// * `params` - Rotation angle
///
/// # Errors
/// * [`EditorError::DecodeError`] if the image cannot be decoded
pub fn rotate(image_bytes: &[u8], params: &RotateParams) -> Result<EditResult, EditorError> {
    let original_hash = sha256_hex(image_bytes);

    let img = load_dynamic_image(image_bytes)?;

    let rotated = match params.angle {
        RotationAngle::Deg90 => img.rotate90(),
        RotationAngle::Deg180 => img.rotate180(),
        RotationAngle::Deg270 => img.rotate270(),
    };

    let out_bytes = encode_png(&rotated)?;
    let edited_hash = sha256_hex(&out_bytes);

    Ok(EditResult {
        width: rotated.width(),
        height: rotated.height(),
        image_bytes: out_bytes,
        record: EditingRecord {
            operation: EditOperation::Rotate,
            parameters: serde_json::json!({
                "angle": params.angle.to_string().parse::<u32>().unwrap(),
            }),
            original_image_hash: original_hash,
            edited_image_hash: edited_hash,
            timestamp: Utc::now().to_rfc3339(),
        },
    })
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Load raw bytes into a `DynamicImage`.
fn load_dynamic_image(bytes: &[u8]) -> Result<DynamicImage, EditorError> {
    image::load_from_memory(bytes)
        .map_err(|e| EditorError::DecodeError(e.to_string()))
}

/// Encode a `DynamicImage` to PNG bytes.
fn encode_png(img: &DynamicImage) -> Result<Vec<u8>, EditorError> {
    let mut buf = std::io::Cursor::new(Vec::new());
    img.write_to(&mut buf, image::ImageFormat::Png)
        .map_err(|e| EditorError::EncodeError(e.to_string()))?;
    Ok(buf.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Create a simple 100x80 RGBA test image as raw PNG bytes.
    fn test_png(width: u32, height: u32) -> Vec<u8> {
        let img = DynamicImage::new_rgba8(width, height);
        let mut buf = std::io::Cursor::new(Vec::new());
        img.write_to(&mut buf, image::ImageFormat::Png).unwrap();
        buf.into_inner()
    }

    // -- Crop tests --------------------------------------------------------

    #[test]
    fn test_crop_success() {
        let png = test_png(100, 80);
        let params = CropParams {
            x: 10,
            y: 10,
            width: 50,
            height: 40,
        };
        let result = crop(&png, &params).unwrap();
        assert_eq!(result.width, 50);
        assert_eq!(result.height, 40);
        assert_eq!(result.record.operation, EditOperation::Crop);
        assert!(!result.record.original_image_hash.is_empty());
        assert!(!result.record.edited_image_hash.is_empty());
        assert_ne!(result.record.original_image_hash, result.record.edited_image_hash);
    }

    #[test]
    fn test_crop_full_image() {
        let png = test_png(100, 80);
        let params = CropParams {
            x: 0,
            y: 0,
            width: 100,
            height: 80,
        };
        let result = crop(&png, &params).unwrap();
        assert_eq!(result.width, 100);
        assert_eq!(result.height, 80);
    }

    #[test]
    fn test_crop_out_of_bounds() {
        let png = test_png(100, 80);
        let params = CropParams {
            x: 50,
            y: 50,
            width: 60,
            height: 40,
        };
        let result = crop(&png, &params);
        assert!(matches!(result, Err(EditorError::CropOutOfBounds { .. })));
    }

    #[test]
    fn test_crop_zero_dimension() {
        let png = test_png(100, 80);
        let params = CropParams {
            x: 0,
            y: 0,
            width: 0,
            height: 50,
        };
        let result = crop(&png, &params);
        assert!(result.is_err());
    }

    // -- Resize tests ------------------------------------------------------

    #[test]
    fn test_resize_success() {
        let png = test_png(100, 80);
        let params = ResizeParams {
            width: 50,
            height: 40,
        };
        let result = resize(&png, &params).unwrap();
        assert_eq!(result.width, 50);
        assert_eq!(result.height, 40);
        assert_eq!(result.record.operation, EditOperation::Resize);
        let p = &result.record.parameters;
        assert_eq!(p["original_width"], 100);
        assert_eq!(p["original_height"], 80);
        assert_eq!(p["new_width"], 50);
        assert_eq!(p["new_height"], 40);
    }

    #[test]
    fn test_resize_upscale() {
        let png = test_png(50, 50);
        let params = ResizeParams {
            width: 200,
            height: 200,
        };
        let result = resize(&png, &params).unwrap();
        assert_eq!(result.width, 200);
        assert_eq!(result.height, 200);
    }

    #[test]
    fn test_resize_zero_dimension() {
        let png = test_png(100, 80);
        let params = ResizeParams {
            width: 0,
            height: 50,
        };
        let result = resize(&png, &params);
        assert!(matches!(
            result,
            Err(EditorError::InvalidResizeDimensions { .. })
        ));
    }

    // -- Rotate tests ------------------------------------------------------

    #[test]
    fn test_rotate_90() {
        let png = test_png(100, 80);
        let params = RotateParams {
            angle: RotationAngle::Deg90,
        };
        let result = rotate(&png, &params).unwrap();
        // 100x80 rotated 90° → 80x100
        assert_eq!(result.width, 80);
        assert_eq!(result.height, 100);
        assert_eq!(result.record.operation, EditOperation::Rotate);
        assert_eq!(result.record.parameters["angle"], 90);
    }

    #[test]
    fn test_rotate_180() {
        let png = test_png(100, 80);
        let params = RotateParams {
            angle: RotationAngle::Deg180,
        };
        let result = rotate(&png, &params).unwrap();
        assert_eq!(result.width, 100);
        assert_eq!(result.height, 80);
    }

    #[test]
    fn test_rotate_270() {
        let png = test_png(100, 80);
        let params = RotateParams {
            angle: RotationAngle::Deg270,
        };
        let result = rotate(&png, &params).unwrap();
        assert_eq!(result.width, 80);
        assert_eq!(result.height, 100);
    }

    // -- Record tests ------------------------------------------------------

    #[test]
    fn test_editing_record_serialization() {
        let png = test_png(100, 80);
        let result = crop(
            &png,
            &CropParams {
                x: 0,
                y: 0,
                width: 50,
                height: 50,
            },
        )
        .unwrap();

        let json = serde_json::to_string_pretty(&result.record).unwrap();
        assert!(json.contains("\"operation\": \"crop\""), "JSON: {}", json);
        assert!(json.contains("\"original_image_hash\""));
        assert!(json.contains("\"edited_image_hash\""));
        assert!(json.contains("\"timestamp\""));

        // Round-trip
        let deserialized: EditingRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.operation, EditOperation::Crop);
    }

    // -- Hash tests --------------------------------------------------------

    #[test]
    fn test_hashes_differ_per_operation() {
        // Use a gradient image so different crop regions produce different pixels
        let img = {
            let mut img = image::RgbaImage::new(100, 80);
            for (x, y, pixel) in img.enumerate_pixels_mut() {
                *pixel = image::Rgba([x as u8, y as u8, (x + y) as u8, 255]);
            }
            let dyn_img = DynamicImage::ImageRgba8(img);
            let mut buf = std::io::Cursor::new(Vec::new());
            dyn_img.write_to(&mut buf, image::ImageFormat::Png).unwrap();
            buf.into_inner()
        };

        let r1 = crop(
            &img,
            &CropParams { x: 0, y: 0, width: 50, height: 50 },
        )
        .unwrap();

        let r2 = crop(
            &img,
            &CropParams { x: 10, y: 10, width: 50, height: 50 },
        )
        .unwrap();

        // Same original
        assert_eq!(r1.record.original_image_hash, r2.record.original_image_hash);
        // Different results
        assert_ne!(r1.record.edited_image_hash, r2.record.edited_image_hash);
    }

    // -- Decode error test -------------------------------------------------

    #[test]
    fn test_decode_garbage_resize() {
        let garbage = b"not an image";
        let result = resize(garbage, &ResizeParams { width: 10, height: 10 });
        assert!(matches!(result, Err(EditorError::DecodeError(_))));
    }

    #[test]
    fn test_decode_garbage_rotate() {
        let garbage = b"not an image";
        let result = rotate(
            garbage,
            &RotateParams { angle: RotationAngle::Deg90 },
        );
        assert!(matches!(result, Err(EditorError::DecodeError(_))));
    }
}
