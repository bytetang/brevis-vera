## ADDED Requirements
### Requirement: C2PA Verification in ZK
The system SHALL verify C2PA provenance inside the ZKVM circuit without revealing signer information.

**Note**: Uses brevis-network/signatures library (ECDSA P-256, Pico-optimized)

#### Scenario: Verify valid C2PA signature
- **WHEN** valid C2PA data (signature + cert chain + image hash) is provided
- **THEN** the ZK circuit SHALL output valid proof

#### Scenario: Reject invalid signature
- **WHEN** tampered image or invalid signature is provided
- **THEN** the ZK circuit SHALL output invalid proof

### Requirement: Crop Proof Generation
The system SHALL generate a ZK proof that a crop operation was applied correctly.

#### Scenario: Generate crop proof
- **WHEN** original image hash, edited image hash, and crop parameters are provided
- **THEN** the system SHALL generate a ZK proof verifying the crop without revealing original dimensions

### Requirement: Resize Proof Generation
The system SHALL generate a ZK proof that a resize operation was applied correctly.

#### Scenario: Generate resize proof
- **WHEN** original dimensions, new dimensions, and image hashes are provided
- **THEN** the system SHALL generate a ZK proof verifying the resize without revealing exact scale

### Requirement: Combined Proof
The system SHALL combine C2PA verification and editing proof into a single ZK proof.

#### Scenario: Generate combined proof
- **WHEN** C2PA data + editing record + original/edited images are provided
- **THEN** the system SHALL generate one ZK proof containing both verifications

### Requirement: Proof Output Format
The system SHALL output a standardized ZK proof format.

#### Scenario: Output proof
- **WHEN** proof generation completes successfully
- **THEN** the system SHALL return proof bytes in Pico-compatible format

### Requirement: Privacy Preservation
The system SHALL NOT reveal sensitive information in the proof.

#### Scenario: Verify privacy
- **WHEN** a ZK proof is generated
- **THEN** the proof SHALL NOT contain: original image, signer identity, exact crop coordinates (only bounds)
