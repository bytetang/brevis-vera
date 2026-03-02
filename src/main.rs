//! Brevis Vera - Main entry point for the REST API server.
//!
//! This binary runs the HTTP server with all API endpoints:
//! - Editing Layer: /api/v1/edit/*
//! - Provenance Layer: /api/v1/provenance/*
//! - ZK Proof Layer: /api/v1/zk/*
//!
//! # Running the server
//!
//! ```bash
//! cargo run --release
//! ```
//!
//! The server will start on http://localhost:8080 by default.

use axum::{
    routing::get,
    Router,
};
use axum::body::Body;
use http::{Request, Response};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

mod api {
    pub mod provenance_api {
        pub use brevis_vera::provenance::api::*;
    }
    pub mod zk_api {
        pub use brevis_vera::zk::api::*;
    }
}

/// Health check endpoint.
async fn health() -> &'static str {
    "Brevis Vera API is running"
}

/// Build the main router with all API endpoints.
fn app_router() -> Router {
    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Logging layer
    let trace = TraceLayer::new_for_http();

    Router::new()
        .route("/", get(health))
        .route("/health", get(health))
        .merge(brevis_vera::editor::api::editing_router())
        .merge(brevis_vera::provenance::api::provenance_router())
        .merge(brevis_vera::zk::api::zk_router())
        .layer(cors)
        .layer(trace)
}

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // Build the application
    let app = app_router();

    // Bind to a port
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("Brevis Vera API server starting on http://localhost:8080");
    tracing::info!("Available endpoints:");
    tracing::info!("  - POST /api/v1/edit/crop");
    tracing::info!("  - POST /api/v1/edit/resize");
    tracing::info!("  - POST /api/v1/edit/rotate");
    tracing::info!("  - POST /api/v1/provenance/extract");
    tracing::info!("  - POST /api/v1/zk/prove");
    tracing::info!("  - GET  /health");
    tracing::info!("Server ready to accept connections");

    // Start the server
    axum::serve(listener, app).await.unwrap();
}
