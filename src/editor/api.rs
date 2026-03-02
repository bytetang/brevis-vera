//! REST API handlers for the Editing Layer.
//!
//! Provides HTTP endpoints for image editing operations:
//! - `POST /api/v1/edit/crop` — Crop an image
//! - `POST /api/v1/edit/resize` — Resize an image
//! - `POST /api/v1/edit/rotate` — Rotate an image
//!
//! # Request Format
//!
//! All endpoints accept JSON with base64-encoded image data and operation
//! parameters. See [`CropRequest`], [`ResizeRequest`], [`RotateRequest`].
//!
//! # Response Format
//!
//! All endpoints return JSON with base64-encoded edited image and an
//! [`EditingRecord`](super::types::EditingRecord). See [`EditResponse`].

use axum::{
    Json, Router,
    extract::DefaultBodyLimit,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use serde::{Deserialize, Serialize};

use super::operations;
use super::types::{CropParams, EditorError, ResizeParams, RotateParams, RotationAngle, EditingRecord};

// ---------------------------------------------------------------------------
// Request/Response types
// ---------------------------------------------------------------------------

/// Request body for the crop endpoint.
///
/// ```json
/// {
///   "image": "<base64-encoded image>",
///   "x": 10, "y": 10, "width": 200, "height": 150
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct CropRequest {
    /// Base64-encoded image data (JPEG or PNG)
    pub image: String,
    /// X coordinate of the top-left corner
    pub x: u32,
    /// Y coordinate of the top-left corner
    pub y: u32,
    /// Width of the crop region
    pub width: u32,
    /// Height of the crop region
    pub height: u32,
}

/// Request body for the resize endpoint.
///
/// ```json
/// {
///   "image": "<base64-encoded image>",
///   "width": 800, "height": 600
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct ResizeRequest {
    /// Base64-encoded image data (JPEG or PNG)
    pub image: String,
    /// Target width
    pub width: u32,
    /// Target height
    pub height: u32,
}

/// Request body for the rotate endpoint.
///
/// ```json
/// {
///   "image": "<base64-encoded image>",
///   "angle": 90
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct RotateRequest {
    /// Base64-encoded image data (JPEG or PNG)
    pub image: String,
    /// Rotation angle: 90, 180, or 270
    pub angle: u32,
}

/// Response body for all editing endpoints.
///
/// ```json
/// {
///   "image": "<base64-encoded edited image>",
///   "width": 200,
///   "height": 150,
///   "record": { ... }
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct EditResponse {
    /// Base64-encoded edited image (PNG)
    pub image: String,
    /// Width of the edited image
    pub width: u32,
    /// Height of the edited image
    pub height: u32,
    /// Editing record for ZK proof generation
    pub record: EditingRecord,
}

/// Error response body.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    /// Error message
    pub error: String,
}

// ---------------------------------------------------------------------------
// Router
// ---------------------------------------------------------------------------

/// Build the Axum router for the editing API.
///
/// Mounts endpoints under `/api/v1/edit/`:
/// - `POST /api/v1/edit/crop`
/// - `POST /api/v1/edit/resize`
/// - `POST /api/v1/edit/rotate`
pub fn editing_router() -> Router {
    Router::new()
        .route("/api/v1/edit/crop", post(handle_crop))
        .route("/api/v1/edit/resize", post(handle_resize))
        .route("/api/v1/edit/rotate", post(handle_rotate))
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024)) // 50 MB
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle `POST /api/v1/edit/crop`.
async fn handle_crop(Json(req): Json<CropRequest>) -> impl IntoResponse {
    let image_bytes = match decode_base64(&req.image) {
        Ok(b) => b,
        Err(e) => return error_response(StatusCode::BAD_REQUEST, &e),
    };

    let params = CropParams {
        x: req.x,
        y: req.y,
        width: req.width,
        height: req.height,
    };

    match operations::crop(&image_bytes, &params) {
        Ok(result) => success_response(result),
        Err(e) => editor_error_response(e),
    }
}

/// Handle `POST /api/v1/edit/resize`.
async fn handle_resize(Json(req): Json<ResizeRequest>) -> impl IntoResponse {
    let image_bytes = match decode_base64(&req.image) {
        Ok(b) => b,
        Err(e) => return error_response(StatusCode::BAD_REQUEST, &e),
    };

    let params = ResizeParams {
        width: req.width,
        height: req.height,
    };

    match operations::resize(&image_bytes, &params) {
        Ok(result) => success_response(result),
        Err(e) => editor_error_response(e),
    }
}

/// Handle `POST /api/v1/edit/rotate`.
async fn handle_rotate(Json(req): Json<RotateRequest>) -> impl IntoResponse {
    let image_bytes = match decode_base64(&req.image) {
        Ok(b) => b,
        Err(e) => return error_response(StatusCode::BAD_REQUEST, &e),
    };

    let angle = match req.angle {
        90 => RotationAngle::Deg90,
        180 => RotationAngle::Deg180,
        270 => RotationAngle::Deg270,
        _ => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "Invalid rotation angle. Must be 90, 180, or 270.",
            )
        }
    };

    let params = RotateParams { angle };

    match operations::rotate(&image_bytes, &params) {
        Ok(result) => success_response(result),
        Err(e) => editor_error_response(e),
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Decode a base64-encoded string to bytes.
fn decode_base64(input: &str) -> Result<Vec<u8>, String> {
    BASE64
        .decode(input)
        .map_err(|e| format!("Invalid base64 image data: {}", e))
}

/// Build a success JSON response.
fn success_response(
    result: super::types::EditResult,
) -> (StatusCode, Json<serde_json::Value>) {
    let resp = EditResponse {
        image: BASE64.encode(&result.image_bytes),
        width: result.width,
        height: result.height,
        record: result.record,
    };
    (StatusCode::OK, Json(serde_json::to_value(resp).unwrap()))
}

/// Convert an [`EditorError`] to an HTTP error response.
fn editor_error_response(err: EditorError) -> (StatusCode, Json<serde_json::Value>) {
    let status = match &err {
        EditorError::CropOutOfBounds { .. } => StatusCode::BAD_REQUEST,
        EditorError::InvalidResizeDimensions { .. } => StatusCode::BAD_REQUEST,
        EditorError::DecodeError(_) => StatusCode::BAD_REQUEST,
        EditorError::EncodeError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        EditorError::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };
    error_response(status, &err.to_string())
}

/// Build a JSON error response.
fn error_response(status: StatusCode, message: &str) -> (StatusCode, Json<serde_json::Value>) {
    (
        status,
        Json(serde_json::json!({ "error": message })),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_valid_base64() {
        let encoded = BASE64.encode(b"hello world");
        let decoded = decode_base64(&encoded).unwrap();
        assert_eq!(decoded, b"hello world");
    }

    #[test]
    fn test_decode_invalid_base64() {
        let result = decode_base64("not valid base64!!!");
        assert!(result.is_err());
    }

    #[test]
    fn test_edit_response_serialization() {
        let resp = EditResponse {
            image: "dGVzdA==".to_string(),
            width: 100,
            height: 80,
            record: EditingRecord {
                operation: super::super::types::EditOperation::Crop,
                parameters: serde_json::json!({"x": 0, "y": 0, "width": 50, "height": 50}),
                original_image_hash: "abc123".to_string(),
                edited_image_hash: "def456".to_string(),
                timestamp: "2026-01-01T00:00:00Z".to_string(),
            },
        };

        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("\"image\""));
        assert!(json.contains("\"record\""));
        assert!(json.contains("\"crop\""));
    }
}
