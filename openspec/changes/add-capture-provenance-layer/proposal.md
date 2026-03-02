# Change: Add Capture & Provenance Layer

## Why
The Capture & Provenance Layer is the foundational component of Brevis Vera. It extracts C2PA metadata from media files and passes it to the ZKVM for privacy-preserving verification. The C2PA signature verification (using ECDSA P-256) happens inside the ZKVM circuit - this ensures the verification is trustless and doesn't leak sensitive information about the signer or certificate chain.

## What Changes
- **NEW**: Add `capture-provenance` capability to extract C2PA metadata
- Accept media files with embedded C2PA provenance metadata
- Parse and extract C2PA data (signature + certificate chain + claims)
- Pass C2PA data to ZK Proof Layer for verification inside ZKVM
- Extract original image data for ZK proof generation

**Note**: C2PA standard uses ECDSA P-256 for signatures. C2PA verification happens inside the ZKVM circuit.

## Impact
- Affected specs: New capability `capture-provenance`
- Affected code: New Rust module under `src/provenance/`
- Dependencies: C2PA metadata parser, ZK circuit for C2PA verification
