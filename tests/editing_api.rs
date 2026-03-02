//! Integration tests for the Editing Layer REST API.
//!
//! Tests the HTTP endpoints using Axum's built-in `oneshot` method
//! from the `tower::ServiceExt` trait.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use brevis_vera::editor::api::editing_router;
use http_body_util::BodyExt;
use image::DynamicImage;
use tower::ServiceExt;

/// Create a test PNG image with a gradient pattern.
fn test_png(width: u32, height: u32) -> Vec<u8> {
    let mut img = image::RgbaImage::new(width, height);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgba([x as u8, y as u8, (x + y) as u8, 255]);
    }
    let dyn_img = DynamicImage::ImageRgba8(img);
    let mut buf = std::io::Cursor::new(Vec::new());
    dyn_img
        .write_to(&mut buf, image::ImageFormat::Png)
        .unwrap();
    buf.into_inner()
}

fn encode_test_image(width: u32, height: u32) -> String {
    BASE64.encode(test_png(width, height))
}

/// Send a POST request with JSON body and return (status, parsed JSON).
async fn post_json(uri: &str, body: serde_json::Value) -> (StatusCode, serde_json::Value) {
    let app = editing_router();

    let req = Request::builder()
        .method("POST")
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&body).unwrap()))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    let status = resp.status();
    let body_bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    (status, json)
}

// -----------------------------------------------------------------------
// Crop endpoint tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_crop_api_success() {
    let body = serde_json::json!({
        "image": encode_test_image(200, 150),
        "x": 10,
        "y": 10,
        "width": 100,
        "height": 80
    });

    let (status, json) = post_json("/api/v1/edit/crop", body).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["width"], 100);
    assert_eq!(json["height"], 80);
    assert!(json["image"].is_string());
    assert_eq!(json["record"]["operation"], "crop");
    assert!(json["record"]["original_image_hash"].is_string());
    assert!(json["record"]["edited_image_hash"].is_string());
    assert!(json["record"]["timestamp"].is_string());
    assert_eq!(json["record"]["parameters"]["x"], 10);
    assert_eq!(json["record"]["parameters"]["y"], 10);
}

#[tokio::test]
async fn test_crop_api_out_of_bounds() {
    let body = serde_json::json!({
        "image": encode_test_image(100, 80),
        "x": 50,
        "y": 50,
        "width": 80,
        "height": 80
    });

    let (status, json) = post_json("/api/v1/edit/crop", body).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(json["error"].as_str().unwrap().contains("out of bounds"));
}

#[tokio::test]
async fn test_crop_api_invalid_base64() {
    let body = serde_json::json!({
        "image": "not_valid_base64!!!",
        "x": 0, "y": 0, "width": 10, "height": 10
    });

    let (status, json) = post_json("/api/v1/edit/crop", body).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(json["error"].as_str().unwrap().contains("base64"));
}

// -----------------------------------------------------------------------
// Resize endpoint tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_resize_api_success() {
    let body = serde_json::json!({
        "image": encode_test_image(200, 150),
        "width": 100,
        "height": 75
    });

    let (status, json) = post_json("/api/v1/edit/resize", body).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["width"], 100);
    assert_eq!(json["height"], 75);
    assert_eq!(json["record"]["operation"], "resize");
    assert_eq!(json["record"]["parameters"]["original_width"], 200);
    assert_eq!(json["record"]["parameters"]["original_height"], 150);
    assert_eq!(json["record"]["parameters"]["new_width"], 100);
    assert_eq!(json["record"]["parameters"]["new_height"], 75);
}

#[tokio::test]
async fn test_resize_api_zero_dimension() {
    let body = serde_json::json!({
        "image": encode_test_image(100, 80),
        "width": 0,
        "height": 50
    });

    let (status, json) = post_json("/api/v1/edit/resize", body).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(json["error"].as_str().unwrap().contains("Invalid resize"));
}

// -----------------------------------------------------------------------
// Rotate endpoint tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_rotate_api_90() {
    let body = serde_json::json!({
        "image": encode_test_image(200, 150),
        "angle": 90
    });

    let (status, json) = post_json("/api/v1/edit/rotate", body).await;
    assert_eq!(status, StatusCode::OK);
    // 200x150 rotated 90° → 150x200
    assert_eq!(json["width"], 150);
    assert_eq!(json["height"], 200);
    assert_eq!(json["record"]["operation"], "rotate");
    assert_eq!(json["record"]["parameters"]["angle"], 90);
}

#[tokio::test]
async fn test_rotate_api_invalid_angle() {
    let body = serde_json::json!({
        "image": encode_test_image(100, 80),
        "angle": 45
    });

    let (status, json) = post_json("/api/v1/edit/rotate", body).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(json["error"].as_str().unwrap().contains("Invalid rotation"));
}

// -----------------------------------------------------------------------
// Test with real image from Capture Layer
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_crop_with_capture_layer_image() {
    let img_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("imgs/test_img.JPG");
    if !img_path.exists() {
        eprintln!("Skipping: test image not found");
        return;
    }

    let image_bytes = std::fs::read(&img_path).unwrap();
    let encoded = BASE64.encode(&image_bytes);

    let body = serde_json::json!({
        "image": encoded,
        "x": 100,
        "y": 100,
        "width": 500,
        "height": 400
    });

    let (status, json) = post_json("/api/v1/edit/crop", body).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["width"], 500);
    assert_eq!(json["height"], 400);
    assert_eq!(json["record"]["operation"], "crop");
}

// -----------------------------------------------------------------------
// Editing record JSON schema validation
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_editing_record_schema() {
    let body = serde_json::json!({
        "image": encode_test_image(100, 80),
        "x": 0, "y": 0, "width": 50, "height": 40
    });

    let (status, json) = post_json("/api/v1/edit/crop", body).await;
    assert_eq!(status, StatusCode::OK);

    let record = &json["record"];

    // Validate required fields per spec
    assert!(record["operation"].is_string(), "operation must be string");
    assert!(record["parameters"].is_object(), "parameters must be object");
    assert!(
        record["original_image_hash"].is_string(),
        "original_image_hash must be string"
    );
    assert!(
        record["edited_image_hash"].is_string(),
        "edited_image_hash must be string"
    );
    assert!(record["timestamp"].is_string(), "timestamp must be string");

    // Validate hash format (hex-encoded SHA-256 = 64 chars)
    let orig_hash = record["original_image_hash"].as_str().unwrap();
    let edit_hash = record["edited_image_hash"].as_str().unwrap();
    assert_eq!(orig_hash.len(), 64, "SHA-256 hash must be 64 hex chars");
    assert_eq!(edit_hash.len(), 64, "SHA-256 hash must be 64 hex chars");
    assert!(
        orig_hash.chars().all(|c| c.is_ascii_hexdigit()),
        "Hash must be hex"
    );
}
