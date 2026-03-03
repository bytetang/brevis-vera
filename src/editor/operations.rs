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
use photon_rs::transform::crop as photon_crop;
use photon_rs::PhotonImage;

use super::types::{
    CropParams, EditOperation, EditResult, EditingRecord, EditorError, ResizeParams, RotateParams,
    RotationAngle, sha256_hex,
};

/// Decode image bytes into a [`PhotonImage`], returning a proper error
/// instead of panicking on invalid input.
///
/// `PhotonImage::new_from_byteslice` internally calls `.unwrap()` and panics
/// when the bytes are not a valid image.  We first validate via the `image`
/// crate, which returns a `Result`, and only then hand the bytes to Photon.
fn decode_photon(image_bytes: &[u8]) -> Result<PhotonImage, EditorError> {
    // Quick validation – this returns Err on garbage input.
    image::load_from_memory(image_bytes).map_err(|e| {
        EditorError::DecodeError(format!("failed to decode image: {e}"))
    })?;

    // Safe to unwrap now – the bytes are valid.
    Ok(PhotonImage::new_from_byteslice(image_bytes.to_vec()))
}

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
    let mut img = decode_photon(image_bytes)?;

    let img_w = img.get_width();
    let img_h = img.get_height();

    // Hash the raw RGBA pixels (not the encoded PNG/JPEG bytes)
    // so that the ZKVM guest can reproduce the same hash from raw pixels.
    let original_raw_pixels = img.get_raw_pixels();
    let original_hash = sha256_hex(&original_raw_pixels);

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

    // Hash the cropped raw RGBA pixels (matching what the ZKVM will compute)
    let cropped_raw_pixels = cropped.get_raw_pixels();
    let edited_hash = sha256_hex(&cropped_raw_pixels);

    let out_bytes = cropped.get_bytes();

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
                "source_width": img_w,
                "source_height": img_h,
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
    if params.width == 0 || params.height == 0 {
        return Err(EditorError::InvalidResizeDimensions {
            width: params.width,
            height: params.height,
        });
    }

    // Decode to raw RGBA pixels for canonical hashing (matches ZKVM guest)
    let src_img = decode_photon(image_bytes)?;
    let original_w = src_img.get_width();
    let original_h = src_img.get_height();
    let original_raw_pixels = src_img.get_raw_pixels();
    let original_hash = sha256_hex(&original_raw_pixels);

    // Perform nearest-neighbor resize on raw RGBA pixels.
    // This matches what the ZKVM guest will re-execute.
    let resized_pixels = nearest_neighbor_resize(
        &original_raw_pixels,
        original_w,
        original_h,
        params.width,
        params.height,
    );
    let edited_hash = sha256_hex(&resized_pixels);

    // Encode the resized image as PNG for output.
    // Build a PhotonImage from the resized raw pixels.
    let resized_photon = PhotonImage::new(
        resized_pixels,
        params.width,
        params.height,
    );
    let out_bytes = resized_photon.get_bytes();

    Ok(EditResult {
        width: params.width,
        height: params.height,
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
    // Decode to raw RGBA pixels for canonical hashing (matches ZKVM guest)
    let src_img = decode_photon(image_bytes)?;
    let src_w = src_img.get_width();
    let src_h = src_img.get_height();
    let original_raw_pixels = src_img.get_raw_pixels();
    let original_hash = sha256_hex(&original_raw_pixels);

    let angle: u32 = match params.angle {
        RotationAngle::Deg90 => 90,
        RotationAngle::Deg180 => 180,
        RotationAngle::Deg270 => 270,
    };

    // Perform rotation on raw RGBA pixels (matches ZKVM guest)
    let (rotated_pixels, out_w, out_h) =
        rotate_pixels(&original_raw_pixels, src_w, src_h, angle);
    let edited_hash = sha256_hex(&rotated_pixels);

    // Encode as PNG for output
    let rotated_photon = PhotonImage::new(rotated_pixels, out_w, out_h);
    let out_bytes = rotated_photon.get_bytes();

    Ok(EditResult {
        width: out_w,
        height: out_h,
        image_bytes: out_bytes,
        record: EditingRecord {
            operation: EditOperation::Rotate,
            parameters: serde_json::json!({
                "angle": angle,
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

// ---------------------------------------------------------------------------
// Pure pixel operations (deterministic, ZKVM-reproducible)
// ---------------------------------------------------------------------------

/// Nearest-neighbor resize on raw RGBA pixel data.
///
/// For each output pixel `(ox, oy)`, sample the source pixel at:
///   `src_x = ox * src_w / dst_w`
///   `src_y = oy * src_h / dst_h`
///
/// This is intentionally simple so the ZKVM guest can reproduce it exactly
/// using only integer arithmetic (no floating point, no interpolation).
pub fn nearest_neighbor_resize(
    pixels: &[u8],
    src_w: u32,
    src_h: u32,
    dst_w: u32,
    dst_h: u32,
) -> Vec<u8> {
    let src_stride = (src_w as usize) * 4;
    let mut out = Vec::with_capacity((dst_w as usize) * (dst_h as usize) * 4);

    for oy in 0..dst_h {
        let sy = (oy as usize * src_h as usize) / dst_h as usize;
        for ox in 0..dst_w {
            let sx = (ox as usize * src_w as usize) / dst_w as usize;
            let src_off = sy * src_stride + sx * 4;
            out.extend_from_slice(&pixels[src_off..src_off + 4]);
        }
    }

    out
}

/// Rotate raw RGBA pixels by 90, 180, or 270 degrees clockwise.
///
/// Returns `(rotated_pixels, new_width, new_height)`.
///
/// Pure pixel permutation — no interpolation, no floating point.
pub fn rotate_pixels(
    pixels: &[u8],
    w: u32,
    h: u32,
    angle: u32,
) -> (Vec<u8>, u32, u32) {
    let (out_w, out_h) = match angle {
        90 | 270 => (h, w),
        180 => (w, h),
        _ => (w, h),
    };

    let src_stride = (w as usize) * 4;
    let dst_stride = (out_w as usize) * 4;
    let mut out = vec![0u8; (out_w as usize) * (out_h as usize) * 4];

    for sy in 0..h {
        for sx in 0..w {
            let (dx, dy) = match angle {
                90 => (h - 1 - sy, sx),
                180 => (w - 1 - sx, h - 1 - sy),
                270 => (sy, w - 1 - sx),
                _ => (sx, sy),
            };
            let src_off = (sy as usize) * src_stride + (sx as usize) * 4;
            let dst_off = (dy as usize) * dst_stride + (dx as usize) * 4;
            out[dst_off..dst_off + 4].copy_from_slice(&pixels[src_off..src_off + 4]);
        }
    }

    (out, out_w, out_h)
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Extract raw RGBA pixel bytes from encoded image bytes (PNG/JPEG).
///
/// Returns `(pixels, width, height)` where `pixels` is a `Vec<u8>` of
/// raw RGBA data (4 bytes per pixel, row-major order).
///
/// This is the canonical pixel representation used for ZK proof hashing.
/// Both the editor and ZKVM guest use this format.
pub fn extract_raw_rgba(image_bytes: &[u8]) -> Result<(Vec<u8>, u32, u32), EditorError> {
    let img = PhotonImage::new_from_byteslice(image_bytes.to_vec());
    Ok((img.get_raw_pixels(), img.get_width(), img.get_height()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::DynamicImage;

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

        // Verify source dimensions are in parameters
        let p = &result.record.parameters;
        assert_eq!(p["source_width"], 100);
        assert_eq!(p["source_height"], 80);
    }

    #[test]
    fn test_crop_hashes_use_raw_pixels() {
        let png = test_png(100, 80);
        let params = CropParams {
            x: 0,
            y: 0,
            width: 50,
            height: 40,
        };
        let result = crop(&png, &params).unwrap();

        // Verify that original hash matches SHA-256 of raw RGBA pixels
        let (raw_pixels, w, h) = extract_raw_rgba(&png).unwrap();
        assert_eq!(w, 100);
        assert_eq!(h, 80);
        let expected_original_hash = sha256_hex(&raw_pixels);
        assert_eq!(result.record.original_image_hash, expected_original_hash);

        // Verify that edited hash matches SHA-256 of cropped raw RGBA pixels
        // (not the PNG-encoded bytes)
        let cropped_img = PhotonImage::new_from_byteslice(result.image_bytes.clone());
        let cropped_raw = cropped_img.get_raw_pixels();
        let expected_edited_hash = sha256_hex(&cropped_raw);
        assert_eq!(result.record.edited_image_hash, expected_edited_hash);
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
