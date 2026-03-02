# Tasks: ZKVM C2PA ECDSA P-256 Verification

## 1. Research and Preparation

- [ ] 1.1 Investigate how to extract ECDSA signature (r, s) from C2PA manifest using c2pa library
- [ ] 1.2 Understand C2PA signing payload (what exactly is signed - image hash, claim, etc.)
- [ ] 1.3 Review existing zk-guest/app/src/main.rs and zk-guest/lib/src/lib.rs structure
- [ ] 1.4 Verify test image exists at test_img_small.JPG

## 2. Host-Side: Signature Data Extraction

- [ ] 2.1 Add signature extraction capability to provenance layer (src/provenance/)
- [ ] 2.2 Create new type for ECDSA signature data (r, s values)
- [ ] 2.3 Parse C2PA manifest to extract signature components
- [ ] 2.4 Pass signature data to ZK prover

## 3. ZK Guest Library: Data Types

- [ ] 3.1 Update CircuitInput in zk-guest/lib/src/lib.rs to include signature data
- [ ] 3.2 Add EcdsaSignature type with r, s components
- [ ] 3.3 Add public key field to CircuitInput

## 4. ZK Guest Program: ECDSA Verification

- [ ] 4.1 Implement ECDSA P-256 verification using Pico SDK precompiles or brevis-network/signatures
- [ ] 4.2 Update verify_c2pa function in zk-guest/app/src/main.rs
- [ ] 4.3 Add public key comparison logic
- [ ] 4.4 Handle verification failure cases

## 5. Integration: Prover to ZKVM

- [ ] 5.1 Update zk-guest/lib/src/lib.rs CircuitInput serialization
- [ ] 5.2 Update src/zk/prover.rs to pass signature data to ZKVM
- [ ] 5.3 Update PublicValuesStruct to include ecdsa_verified flag

## 6. Testing

- [ ] 6.1 Build ZK guest with cargo pico build
- [ ] 6.2 Run test with test_img_small.JPG using provided public key
- [ ] 6.3 Verify ECDSA verification returns correct result
- [ ] 6.4 Test invalid signature case
- [ ] 6.5 Test missing signature data fallback

## 7. Verification and Documentation

- [ ] 7.1 Verify proof output includes cryptographic verification result
- [ ] 7.2 Update circuit verification in src/zk/circuits.rs if needed
- [ ] 7.3 Document the implementation
