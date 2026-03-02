# Specification: ZKVM ECDSA P-256 Verification

This specification defines the requirements for ECDSA P-256 signature verification inside the ZKVM for C2PA identity authentication.

## ADDED Requirements

### Requirement: ECDSA signature verification inside ZKVM

The ZK guest program SHALL verify ECDSA P-256 signatures inside the ZKVM to provide cryptographic proof of C2PA identity authentication.

#### Scenario: Valid signature with correct public key
- **WHEN** the ZKVM receives a valid ECDSA P-256 signature (r, s), the correct public key, and the image hash that was signed
- **THEN** the verification SHALL return true and the proof SHALL indicate successful cryptographic verification

#### Scenario: Invalid signature with correct public key
- **WHEN** the ZKVM receives an invalid ECDSA P-256 signature (r, s), the correct public key, and the image hash
- **THEN** the verification SHALL return false and the proof SHALL indicate verification failure

#### Scenario: Signature verification with wrong public key
- **WHEN** the ZKVM receives a valid ECDSA P-256 signature but with a mismatched public key
- **THEN** the verification SHALL return false since the signature was not made by the expected key

#### Scenario: Missing signature data
- **WHEN** the ZKVM receives incomplete signature data (missing r, s, or public key)
- **THEN** the verification SHALL return false and indicate missing input data

### Requirement: Integration with C2PA structural validation

The ECDSA cryptographic verification SHALL work together with the existing C2PA structural validation to provide complete provenance verification.

#### Scenario: Both structural and cryptographic validation pass
- **WHEN** C2PA manifest has valid structure AND ECDSA signature is valid
- **THEN** the combined verification SHALL return true

#### Scenario: Structural valid but cryptographic fails
- **WHEN** C2PA manifest structure is valid but ECDSA signature verification fails
- **THEN** the combined verification SHALL return false (cryptographic verification is authoritative)

#### Scenario: Cryptographic verification disabled
- **WHEN** no signature data is provided to the ZKVM
- **THEN** the system SHALL fall back to structural-only verification (existing behavior)

### Requirement: Public values output

The ZK proof SHALL output whether ECDSA cryptographic verification passed as part of the public values.

#### Scenario: Verification result in public values
- **WHEN** ECDSA verification completes inside ZKVM
- **THEN** the result SHALL be included in the PublicValuesStruct that is committed to the proof

### Requirement: Test with known public key

The implementation SHALL be testable using the provided test image and public key.

#### Scenario: Test image verification
- **GIVEN** test image `test_img_small.JPG` with known public key `0460783afb3dba96bd37568481744eb0d8c0257261b8bc16dc96a6f50a867ea4bba3c6d8da159c60e5935399a394764baa6298eed36427269fb5a23c032d8815e9`
- **WHEN** the ZKVM verifies the C2PA signature using this public key
- **THEN** the verification SHALL succeed if the signature is valid for that image

## MODIFIED Requirements

### Requirement: zk-proof capability enhanced

The existing zk-proof capability SHALL be modified to include cryptographic ECDSA verification.

**Updated Description:** The ZK Proof Generation Layer generates zero-knowledge proofs that verify both:
1. **C2PA Provenance (Cryptographic)**: The original image hash was signed by the expected ECDSA P-256 public key (verified inside ZKVM)
2. **Editing Proof**: The claimed edits were applied correctly without revealing sensitive details

#### Scenario: Combined proof with cryptographic verification
- **WHEN** generating a combined proof with C2PA data
- **THEN** the proof SHALL attest to actual ECDSA signature verification inside ZKVM (not just structural checks)
