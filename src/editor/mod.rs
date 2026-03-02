//! Editing Layer
//!
//! The Editing Layer is the second stage of the Brevis Vera pipeline.
//! It applies image transformations (crop, resize, rotate) to media files
//! and records each operation for ZK proof generation.
//!
//! # Architecture
//!
//! ```text
//! HTTP Request ─→ API Handler ─→ Operation ─→ EditResult
//!                                    │              │
//!                                    │         ┌────┴────┐
//!                                    └─────►   │ Editing │
//!                                              │ Record  │
//!                                              └─────────┘
//! ```
//!
//! # Submodules
//!
//! - [`operations`] — Core image editing functions (crop, resize, rotate)
//! - [`api`] — REST API handlers and router (Axum)
//! - [`types`] — Data structures, parameters, records, and errors
//!
//! # REST API Endpoints
//!
//! | Method | Path | Description |
//! |--------|------|-------------|
//! | POST | `/api/v1/edit/crop` | Crop an image |
//! | POST | `/api/v1/edit/resize` | Resize an image |
//! | POST | `/api/v1/edit/rotate` | Rotate an image (90/180/270°) |
//!
//! # Editing Record
//!
//! Each operation produces an [`EditingRecord`] containing:
//! - Operation type and parameters
//! - SHA-256 hashes of original and edited images
//! - Timestamp
//!
//! This record is passed to the ZK Proof Layer for privacy-preserving
//! proof generation.
//!
//! # Example (direct API)
//!
//! ```no_run
//! use brevis_vera::editor::operations;
//! use brevis_vera::editor::types::{CropParams, ResizeParams};
//!
//! // Load image bytes
//! let image_bytes = std::fs::read("image.png").unwrap();
//!
//! // Crop
//! let cropped = operations::crop(&image_bytes, &CropParams {
//!     x: 10, y: 10, width: 200, height: 150,
//! }).unwrap();
//! println!("Cropped to {}x{}", cropped.width, cropped.height);
//!
//! // Resize
//! let resized = operations::resize(&image_bytes, &ResizeParams {
//!     width: 800, height: 600,
//! }).unwrap();
//! println!("Resized to {}x{}", resized.width, resized.height);
//! ```

pub mod api;
pub mod operations;
pub mod types;

pub use api::editing_router;
pub use types::*;
