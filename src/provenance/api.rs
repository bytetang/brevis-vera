//! REST API handlers for the Provenance Layer.
//!
//! Provides HTTP endpoints for provenance extraction:
//! - `POST /api/v1/provenance/extract` — Extract C2PA metadata from image
//!
//! # Request Format
//!
//! Accepts JSON with base64-encoded image data. See [`ProvenanceExtractRequest`].
//!
//! # Response Format
//!
//! Returns JSON with extracted metadata and image hash. See [`ProvenanceExtractResponse`].

use axum::{
    Json, Router,
    extract::DefaultBodyLimit,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use serde::{Deserialize, Serialize};
use std::io::Write;
use tempfile::NamedTempFile;

use crate::provenance::ProvenanceProcessor;
use crate::provenance::types::MediaFormat;

// ---------------------------------------------------------------------------
// Request/Response types
// ---------------------------------------------------------------------------

/// Request body for the provenance extract endpoint.
///
/// ```json
/// {
///   "image": "<base64-encoded image>"
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct ProvenanceExtractRequest {
    /// Base64-encoded image data (JPEG or PNG)
    pub image: String,
}

/// Response body for the provenance extract endpoint.
///
/// ```json
/// {
///   "has_c2pa": true,
///   "c2pa_metadata": { ... },
///   "image_hash": "abc123...",
///   "format": "Jpeg"
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct ProvenanceExtractResponse {
    /// Whether C2PA metadata was found in the image
    pub has_c2pa: bool,
    /// C2PA metadata (null if not present)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c2pa_metadata: Option<serde_json::Value>,
    /// SHA-256 hash of the image (hex-encoded)
    pub image_hash: String,
    /// Image format (Jpeg or Png)
    pub format: String,
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

/// Build the Axum router for the provenance API.
///
/// Mounts endpoints under `/api/v1/provenance/`:
/// - `POST /api/v1/provenance/extract`
pub fn provenance_router() -> Router {
    Router::new()
        .route("/api/v1/provenance/extract", post(handle_extract))
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024)) // 50 MB
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle `POST /api/v1/provenance/extract`.
async fn handle_extract(Json(req): Json<ProvenanceExtractRequest>) -> impl IntoResponse {
    // Decode base64 image
    let image_bytes = match BASE64.decode(&req.image) {
        Ok(b) => b,
        Err(e) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                &format!("Invalid base64 image data: {}", e),
            )
        }
    };

    // Detect format from magic bytes to determine file extension.
    // c2pa::ManifestStore::from_file relies on the file extension to parse
    // the correct media container, so we must use the right suffix.
    let suffix = match crate::provenance::reader::detect_format(&image_bytes) {
        Ok(MediaFormat::Jpeg) => ".jpg",
        Ok(MediaFormat::Png) => ".png",
        Err(_) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "Unsupported image format (only JPEG and PNG are supported)",
            )
        }
    };

    // Write to temp file (c2pa crate requires file path)
    let mut temp_file: NamedTempFile = match tempfile::Builder::new()
        .suffix(suffix)
        .tempfile()
    {
        Ok(f) => f,
        Err(e) => {
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Failed to create temp file: {}", e),
            )
        }
    };

    if let Err(e) = temp_file.write_all(&image_bytes) {
        return error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("Failed to write temp file: {}", e),
        );
    }

    // Need to flush to ensure data is written
    if let Err(e) = temp_file.flush() {
        return error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("Failed to flush temp file: {}", e),
        );
    }

    // Get the path - we need to keep the temp file alive
    let temp_path = temp_file.path().to_path_buf();
    let processor = ProvenanceProcessor::new();

    // Process the image
    let result = match processor.process(&temp_path) {
        Ok(r) => r,
        Err(e) => {
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Failed to process image: {}", e),
            )
        }
    };

    // Build response
    let has_c2pa = result.c2pa_metadata.is_some();
    let c2pa_metadata = result.c2pa_metadata.map(|m| {
        serde_json::json!({
            "active_manifest": m.active_manifest,
            "claim_generator": m.claim_generator,
            "title": m.title,
            "format": m.format,
            "signature_info": m.signature_info,
            "assertions": m.assertions,
            "ingredients": m.ingredients,
        })
    });

    let format_str = match result.format {
        MediaFormat::Jpeg => "Jpeg",
        MediaFormat::Png => "Png",
    };

    let response = ProvenanceExtractResponse {
        has_c2pa,
        c2pa_metadata,
        image_hash: result.image_hash.to_hex(),
        format: format_str.to_string(),
    };

    (StatusCode::OK, Json(serde_json::to_value(response).unwrap()))
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

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
    fn test_extract_request_deserialization() {
        let json = r#"{"image": "SGVsbG8gV29ybGQh"}"#;
        let req: ProvenanceExtractRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.image, "SGVsbG8gV29ybGQh");
    }

    #[test]
    fn test_extract_response_serialization() {
        let resp = ProvenanceExtractResponse {
            has_c2pa: true,
            c2pa_metadata: Some(serde_json::json!({
                "claim_generator": "Test/1.0"
            })),
            image_hash: "abc123".to_string(),
            format: "Jpeg".to_string(),
        };

        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("\"has_c2pa\":true"));
        assert!(json.contains("abc123"));
    }
}
