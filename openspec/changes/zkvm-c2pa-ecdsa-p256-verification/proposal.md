# Change: ZKVM C2PA ECDSA P-256 Verification

## Why
Currently, the cryptographic signature verification for C2PA provenance is performed on the host side using the `c2pa` library. This creates a significant security vulnerability: a malicious host could:
1. Skip signature verification entirely
2. Fake valid verification results
3. Return incorrect verification outcomes to the ZKVM

Moving ECDSA P-256 signature verification inside the ZKVM ensures that:
- Verification happens in a trusted execution environment
- The proof attests to actual cryptographic verification, not just structural checks
- No malicious host can bypass the security-critical signature validation

## What Changes
- **MODIFY**: Enhance C2PA verification in ZKVM to include ECDSA P-256 cryptographic verification
- **NEW**: Add ECDSA P-256 signature verification circuit to ZK guest program
- **NEW**: Add host-side integration to pass signature data to ZKVM
- **NEW**: Add test with `test_img_small.JPG` using provided public key

## Capabilities

### New Capabilities
- `zkvm-ecdsa-p256-verification`: ECDSA P-256 signature verification inside ZKVM for C2PA identity authentication

### Modified Capabilities
- `zk-proof`: The existing zk-proof capability will be enhanced to include cryptographic verification (not just structural validation)

## Impact
- Affected specs: `zk-proof` (modified), new `zkvm-ecdsa-p256-verification`
- Affected code:
  - `zk-guest/app/src/main.rs` - add ECDSA verification logic
  - `zk-guest/lib/src/lib.rs` - add signature data types
  - `src/zk/circuits.rs` - update verification circuit
  - `src/zk/prover.rs` - integrate host-side signature data
- Dependencies: `p256` crate, `brevis-network/signatures`
- Test file: `test_img_small.JPG` with public key `0460783afb3dba96bd37568481744eb0d8c0257261b8bc16dc96a6f50a867ea4bba3c6d8da159c60e5935399a394764baa6298eed36427269fb5a23c032d8815e9`

## Required Learning Resources

### Core Documentation
| Resource | URL | Purpose |
|----------|-----|---------|
| Pico ZKVM Docs | https://pico-docs.brevis.network/ | Main documentation |
| Pico GitHub | https://github.com/brevis-network/pico | Source code & examples |
| p256 crate | https://docs.rs/p256/latest/p256/ | ECDSA P-256 implementation |
| brevis-network/signatures | https://github.com/brevis-network/signatures | ECDSA P-256 circuits |

### Topics to Study
1. **ECDSA P-256 Signature Verification**
   - Signature structure (r, s values)
   - Public key format (uncompressed 04 prefix)
   - Message hashing

2. **ZKVM Integration**
   - How to call cryptographic functions in ZKVM
   - Input/output serialization for signature data
   - Constraint system for ECDSA

3. **C2PA Signature Data**
   - Extract signature (r, s) from C2PA manifest
   - Extract message/hash that was signed
   - Handle C2PA-specific signing format
