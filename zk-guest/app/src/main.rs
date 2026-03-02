//! Brevis Vera ZKVM Guest Program
//!
//! This program runs inside the Pico ZKVM to generate zero-knowledge proofs
//! that verify both C2PA provenance and editing operations.
//!
//! # What runs inside the ZKVM
//!
//! 1. Read `CircuitInput` from the host via `pico_sdk::io::read_as()`
//! 2. Verify C2PA data structure (structural validation; cryptographic
//!    signature verification is performed by the host using the `c2pa` library)
//! 3. Verify editing hash chain (each operation's output = next input)
//! 4. Commit `PublicValuesStruct` via `pico_sdk::io::commit_bytes()`
//!
//! # Building
//!
//! ```bash
//! cd zk-guest/app
//! cargo pico build
//! ```
//!
//! This produces a RISC-V ELF binary at `elf/riscv32im-pico-zkvm-elf`
//! which is loaded by the host prover.
//!
//! # Privacy guarantees
//!
//! The proof reveals ONLY the `PublicValuesStruct` fields:
//! - Whether C2PA was verified (bool)
//! - Whether editing was verified (bool)
//! - Original image hash
//! - Edited image hash
//! - Number of editing operations
//!
//! The proof does NOT reveal:
//! - C2PA signer identity or certificate details
//! - Exact crop coordinates or resize dimensions
//! - Original image content

#![no_main]

use alloy_sol_types::SolValue;
use brevis_vera_zk_lib::{
    CircuitInput, EditingRecordData, OperationParams, PublicValuesStruct,
};
use pico_sdk::io::{commit_bytes, read_as};

pico_sdk::entrypoint!(main);

pub fn main() {
    // -----------------------------------------------------------------------
    // 1. Read input from the host prover
    // -----------------------------------------------------------------------
    let input: CircuitInput = read_as();

    // -----------------------------------------------------------------------
    // 2. Verify C2PA provenance
    // -----------------------------------------------------------------------
    let c2pa_verified: bool = if let Some(ref c2pa) = input.c2pa_data {
        verify_c2pa(c2pa)
    } else {
        false
    };

    // -----------------------------------------------------------------------
    // 3. Verify editing operations (hash chain)
    // -----------------------------------------------------------------------
    let (editing_verified, num_operations) = if !input.editing_records.is_empty() {
        verify_editing(
            &input.original_image_hash,
            input.edited_image_hash.as_deref().unwrap_or(""),
            &input.editing_records,
        )
    } else {
        (false, 0u32)
    };

    // -----------------------------------------------------------------------
    // 4. Commit public values
    // -----------------------------------------------------------------------
    let original_hash_bytes = hex_to_bytes32(&input.original_image_hash);
    let edited_hash_bytes = input
        .edited_image_hash
        .as_deref()
        .map(hex_to_bytes32)
        .unwrap_or([0u8; 32]);

    let public_values = PublicValuesStruct {
        c2pa_verified: if c2pa_verified { 1 } else { 0 },
        editing_verified: if editing_verified { 1 } else { 0 },
        original_image_hash: original_hash_bytes.into(),
        edited_image_hash: edited_hash_bytes.into(),
        num_operations,
    };

    commit_bytes(&public_values.abi_encode());
}

// ---------------------------------------------------------------------------
// C2PA verification logic
// ---------------------------------------------------------------------------

/// Verify C2PA provenance data (structural validation).
///
/// Checks that the C2PA manifest data has the required fields and
/// a recognised signing algorithm.  Cryptographic signature verification
/// (ECDSA / RSA-PSS, any algorithm) is performed by the host-side `c2pa`
/// library *before* the data reaches the guest.  This keeps the guest
/// binary small, algorithm-agnostic, and free of complex crypto deps.
fn verify_c2pa(c2pa: &brevis_vera_zk_lib::C2paInputData) -> bool {
    // ----- structural checks -----
    if c2pa.active_manifest.is_empty() {
        return false;
    }
    if c2pa.claim_generator.is_empty() {
        return false;
    }

    // Algorithm must be a known value (case-insensitive)
    if let Some(ref alg) = c2pa.algorithm {
        let alg_upper = alg.to_uppercase();
        if alg_upper != "ES256"
            && alg_upper != "ES384"
            && alg_upper != "ES512"
            && alg_upper != "PS256"
            && alg_upper != "PS384"
            && alg_upper != "PS512"
            && alg_upper != "ED25519"
        {
            return false;
        }
    }

    true
}

// ---------------------------------------------------------------------------
// Editing verification logic
// ---------------------------------------------------------------------------

/// Verify the editing operations form a valid hash chain.
///
/// Checks:
/// 1. First operation's input_hash == original_image_hash
/// 2. Each operation has valid parameters
/// 3. output_hash of op N == input_hash of op N+1
/// 4. Last operation's output_hash == edited_image_hash
/// 5. Input and output hashes differ for each operation
fn verify_editing(
    original_hash: &str,
    edited_hash: &str,
    records: &[EditingRecordData],
) -> (bool, u32) {
    if records.is_empty() {
        return (false, 0);
    }

    // Check chain starts with original image
    if records[0].input_hash != original_hash {
        return (false, 0);
    }

    let num_records = records.len() as u32;

    for i in 0..records.len() {
        let record = &records[i];

        // Hashes must differ (operation must change the image)
        if record.input_hash == record.output_hash {
            return (false, 0);
        }

        // Validate operation-specific parameters
        if !verify_operation_params(record) {
            return (false, 0);
        }

        // Check hash chain continuity
        if i + 1 < records.len() && record.output_hash != records[i + 1].input_hash {
            return (false, 0);
        }
    }

    // Check chain ends with edited image
    let last = &records[records.len() - 1];
    if last.output_hash != edited_hash {
        return (false, 0);
    }

    (true, num_records)
}

/// Validate operation-specific parameters.
fn verify_operation_params(record: &EditingRecordData) -> bool {
    match &record.params {
        OperationParams::Crop {
            width, height, ..
        } => {
            // Crop dimensions must be non-zero
            *width > 0 && *height > 0
        }
        OperationParams::Resize { width, height } => {
            // Resize dimensions must be non-zero
            *width > 0 && *height > 0
        }
        OperationParams::Rotate { angle } => {
            // Angle must be 90, 180, or 270
            *angle == 90 || *angle == 180 || *angle == 270
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Convert a hex string to a 32-byte array.
///
/// If the hex string is shorter than 64 chars, the result is zero-padded.
/// If longer, only the first 32 bytes are used.
fn hex_to_bytes32(hex: &str) -> [u8; 32] {
    let mut result = [0u8; 32];
    let bytes: Vec<u8> = (0..hex.len())
        .step_by(2)
        .filter_map(|i| {
            if i + 2 <= hex.len() {
                u8::from_str_radix(&hex[i..i + 2], 16).ok()
            } else {
                None
            }
        })
        .collect();

    let len = bytes.len().min(32);
    result[..len].copy_from_slice(&bytes[..len]);
    result
}
