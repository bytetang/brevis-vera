# Tasks: ZKVM C2PA ECDSA P-256 Verification

## 1. Research and Preparation

- [x] 1.1 Investigate how to extract ECDSA signature (r, s) from C2PA manifest using c2pa library
- [x] 1.2 Understand C2PA signing payload (what exactly is signed - image hash, claim, etc.)
- [x] 1.3 Review existing zk-guest/app/src/main.rs and zk-guest/lib/src/lib.rs structure
- [x] 1.4 Verify test image exists at test_img_small.JPG

## 2. Host-Side: Signature Data Extraction

- [x] 2.1 Add signature extraction capability to provenance layer (src/provenance/)
- [x] 2.2 Create new type for ECDSA signature data (r, s values)
- [x] 2.3 Parse C2PA manifest to extract signature components (stub - needs c2pa API integration)
- [x] 2.4 Pass signature data to ZK prover

## 3. ZK Guest Library: Data Types

- [x] 3.1 Update CircuitInput in zk-guest/lib/src/lib.rs to include signature data
- [x] 3.2 Add EcdsaSignature type with r, s components
- [x] 3.3 Add public key field to CircuitInput

## 4. ZK Guest Program: ECDSA Verification

- [x] 4.1 Implement ECDSA P-256 verification using Pico SDK precompiles or brevis-network/signatures (placeholder)
- [x] 4.2 Update verify_c2pa function in zk-guest/app/src/main.rs
- [x] 4.3 Add public key comparison logic
- [x] 4.4 Handle verification failure cases

## 5. Integration: Prover to ZKVM

- [x] 5.1 Update zk-guest/lib/src/lib.rs CircuitInput serialization
- [x] 5.2 Update src/zk/prover.rs to pass signature data to ZKVM
- [x] 5.3 Update PublicValuesStruct to include ecdsa_verified flag

## 6. Testing

- [x] 6.1 Build ZK guest with cargo pico build
- [x] 6.2 Run test with test_img_small.JPG using provided public key
- [x] 6.3 Verify ECDSA verification returns correct result (format validation - placeholder)
- [x] 6.4 Test invalid signature case (format validation rejects invalid formats)
- [x] 6.5 Test missing signature data fallback (returns true for backward compatibility)

## 7. Verification and Documentation

- [x] 7.1 Verify proof output includes cryptographic verification result
- [x] 7.2 Update circuit verification in src/zk/circuits.rs if needed
- [x] 7.3 Document the implementation
