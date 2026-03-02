//! REST API handlers for the ZK Proof Layer.
//!
//! Provides HTTP endpoints for ZK proof generation:
//! - `POST /api/v1/zk/prove` — Generate ZK proof for C2PA and/or editing
//!
//! # Request Format
//!
//! Accepts JSON with proof input data. See [`ZkProveRequest`].
//!
//! # Response Format
//!
//! Returns JSON with generated proof. See [`ZkProveResponse`].

use axum::{
    Json, Router,
    extract::DefaultBodyLimit,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
};
use serde::{Deserialize, Serialize};

use crate::zk::prover::{SimulatedProver, ZkProver};
use crate::zk::types::{
    C2paProofInput, CombinedProofInput, EditingProofInput, EditingRecordInput,
    ZkProof, ZkError,
};
use crate::provenance::types::C2paVerificationData;

// ---------------------------------------------------------------------------
// Request/Response types
// ---------------------------------------------------------------------------

/// Request body for the ZK prove endpoint.
///
/// ```json
/// {
///   "proof_type": "combined",
///   "c2pa_data": { ... },
///   "original_image_hash": "abc123...",
///   "editing_records": [...],
///   "edited_image_hash": "def456..."
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct ZkProveRequest {
    /// Type of proof to generate: "c2pa", "editing", or "combined"
    pub proof_type: String,
    /// C2PA verification data (required for "c2pa" or "combined")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c2pa_data: Option<C2paDataInput>,
    /// SHA-256 hash of the original image (hex-encoded)
    pub original_image_hash: String,
    /// Editing records (required for "editing" or "combined")
    #[serde(default)]
    pub editing_records: Vec<EditingRecordInput>,
    /// SHA-256 hash of the edited image (required for "editing" or "combined")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_image_hash: Option<String>,
}

/// Simplified C2PA data input for API.
#[derive(Debug, Deserialize, Serialize)]
pub struct C2paDataInput {
    pub active_manifest: String,
    pub claim_generator: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_info: Option<SignatureInfoInput>,
    #[serde(default)]
    pub assertions: Vec<serde_json::Value>,
}

/// Simplified signature info input for API.
#[derive(Debug, Deserialize, Serialize)]
pub struct SignatureInfoInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alg: Option<String>,
}

/// Response body for the ZK prove endpoint.
///
/// ```json
/// {
///   "proof_base64": "...",
///   "public_inputs": { ... },
///   "metadata": { ... }
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct ZkProveResponse {
    /// Base64-encoded proof bytes
    pub proof_base64: String,
    /// Public inputs visible to verifier
    pub public_inputs: PublicInputsResponse,
    /// Proof generation metadata
    pub metadata: MetadataResponse,
}

/// Simplified public inputs for API response.
#[derive(Debug, Serialize)]
pub struct PublicInputsResponse {
    pub c2pa_verified: bool,
    pub editing_verified: bool,
    pub original_image_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_image_hash: Option<String>,
    pub operations_applied: Vec<String>,
}

/// Simplified metadata for API response.
#[derive(Debug, Serialize)]
pub struct MetadataResponse {
    pub prover_type: String,
    pub generation_time_ms: u64,
    pub timestamp: String,
    pub proof_version: String,
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

/// Build the Axum router for the ZK API.
///
/// Mounts endpoints under `/api/v1/zk/`:
/// - `POST /api/v1/zk/prove`
pub fn zk_router() -> Router {
    Router::new()
        .route("/api/v1/zk/prove", post(handle_prove))
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024)) // 50 MB
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle `POST /api/v1/zk/prove`.
async fn handle_prove(Json(req): Json<ZkProveRequest>) -> impl IntoResponse {
    // Validate request
    if req.original_image_hash.is_empty() {
        return error_response(
            StatusCode::BAD_REQUEST,
            "original_image_hash is required",
        );
    }

    if req.original_image_hash.len() != 64 {
        return error_response(
            StatusCode::BAD_REQUEST,
            "original_image_hash must be 64 characters (SHA-256 hex)",
        );
    }

    // Create prover (using SimulatedProver for now)
    let prover = SimulatedProver::new();

    // Generate proof based on type
    let result = match req.proof_type.as_str() {
        "c2pa" => generate_c2pa_proof(&prover, &req),
        "editing" => generate_editing_proof(&prover, &req),
        "combined" => generate_combined_proof(&prover, &req),
        _ => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "proof_type must be 'c2pa', 'editing', or 'combined'",
            )
        }
    };

    match result {
        Ok(proof) => {
            // Build response
            let proof_base64 = base64::Engine::encode(
                &base64::engine::general_purpose::STANDARD,
                &proof.proof_bytes,
            );

            let response = ZkProveResponse {
                proof_base64,
                public_inputs: PublicInputsResponse {
                    c2pa_verified: proof.public_inputs.c2pa_verified,
                    editing_verified: proof.public_inputs.editing_verified,
                    original_image_hash: proof.public_inputs.original_image_hash,
                    edited_image_hash: proof.public_inputs.edited_image_hash,
                    operations_applied: proof
                        .public_inputs
                        .operations_applied
                        .iter()
                        .map(|op| op.to_string())
                        .collect(),
                },
                metadata: MetadataResponse {
                    prover_type: proof.metadata.prover_type,
                    generation_time_ms: proof.metadata.generation_time_ms,
                    timestamp: proof.metadata.timestamp,
                    proof_version: proof.metadata.proof_version,
                },
            };

            (StatusCode::OK, Json(serde_json::to_value(response).unwrap()))
        }
        Err(e) => error_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
    }
}

/// Generate a C2PA-only proof.
fn generate_c2pa_proof(
    prover: &SimulatedProver,
    req: &ZkProveRequest,
) -> Result<ZkProof, ZkError> {
    let c2pa_data = req
        .c2pa_data
        .as_ref()
        .ok_or_else(|| ZkError::MissingInput("c2pa_data is required for c2pa proof".to_string()))?;

    let c2pa_verification_data = C2paVerificationData {
        active_manifest: c2pa_data.active_manifest.clone(),
        claim_generator: c2pa_data.claim_generator.clone(),
        signature_info: c2pa_data.signature_info.as_ref().map(|s| {
            crate::provenance::types::SignatureInfo {
                issuer: s.issuer.clone(),
                time: s.time.clone(),
                cert_serial_number: s.cert_serial_number.clone(),
                alg: s.alg.clone(),
            }
        }),
        assertions: vec![], // Simplified for now
    };

    let input = C2paProofInput {
        c2pa_data: c2pa_verification_data,
        image_hash: req.original_image_hash.clone(),
    };

    prover.prove_c2pa(&input)
}

/// Generate an editing-only proof.
fn generate_editing_proof(
    prover: &SimulatedProver,
    req: &ZkProveRequest,
) -> Result<ZkProof, ZkError> {
    if req.editing_records.is_empty() {
        return Err(ZkError::MissingInput(
            "editing_records is required for editing proof".to_string(),
        ));
    }

    let edited_image_hash = req
        .edited_image_hash
        .as_ref()
        .ok_or_else(|| ZkError::MissingInput("edited_image_hash is required for editing proof".to_string()))?
        .clone();

    let input = EditingProofInput {
        original_image_hash: req.original_image_hash.clone(),
        edited_image_hash,
        editing_records: req.editing_records.clone(),
    };

    prover.prove_editing(&input)
}

/// Generate a combined proof (C2PA + editing).
fn generate_combined_proof(
    prover: &SimulatedProver,
    req: &ZkProveRequest,
) -> Result<ZkProof, ZkError> {
    let c2pa_data = req.c2pa_data.as_ref().map(|c| {
        C2paVerificationData {
            active_manifest: c.active_manifest.clone(),
            claim_generator: c.claim_generator.clone(),
            signature_info: c.signature_info.as_ref().map(|s| {
                crate::provenance::types::SignatureInfo {
                    issuer: s.issuer.clone(),
                    time: s.time.clone(),
                    cert_serial_number: s.cert_serial_number.clone(),
                    alg: s.alg.clone(),
                }
            }),
            assertions: vec![],
        }
    });

    let input = CombinedProofInput {
        c2pa_data,
        original_image_hash: req.original_image_hash.clone(),
        editing_records: req.editing_records.clone(),
        edited_image_hash: req.edited_image_hash.clone(),
    };

    prover.prove_combined(&input)
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
    fn test_prove_request_deserialization() {
        let json = r#"{
            "proof_type": "combined",
            "original_image_hash": "abc123def456abc123def456abc123def456abc123def456abc123def456ab",
            "c2pa_data": {
                "active_manifest": "urn:test",
                "claim_generator": "Test/1.0"
            },
            "editing_records": [],
            "edited_image_hash": null
        }"#;
        let req: ZkProveRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.proof_type, "combined");
        assert_eq!(req.original_image_hash.len(), 64);
    }

    #[test]
    fn test_prove_response_serialization() {
        let resp = ZkProveResponse {
            proof_base64: "SGVsbG8=".to_string(),
            public_inputs: PublicInputsResponse {
                c2pa_verified: true,
                editing_verified: false,
                original_image_hash: "abc123".to_string(),
                edited_image_hash: None,
                operations_applied: vec![],
            },
            metadata: MetadataResponse {
                prover_type: "simulated".to_string(),
                generation_time_ms: 100,
                timestamp: "2026-01-01T00:00:00Z".to_string(),
                proof_version: "0.1.0".to_string(),
            },
        };

        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("\"c2pa_verified\":true"));
    }
}
