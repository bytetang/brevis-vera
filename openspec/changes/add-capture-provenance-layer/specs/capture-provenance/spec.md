## ADDED Requirements
### Requirement: Media File Acceptance
The system SHALL accept media files (JPEG, PNG) with embedded C2PA/ECDSA provenance metadata.

#### Scenario: Valid JPEG with provenance
- **WHEN** a JPEG image with valid C2PA metadata is submitted
- **THEN** the system SHALL parse the metadata and extract signature data

#### Scenario: PNG with metadata
- **WHEN** a PNG image with embedded metadata is submitted
- **THEN** the system SHALL extract the metadata structure

### Requirement: C2PA Metadata Extraction
The system SHALL extract C2PA metadata (signature + certificate chain + claims) for ZKVM verification.

**Note**: C2PA standard uses ECDSA P-256 for digital signatures. C2PA verification happens inside the ZKVM circuit for privacy-preserving verification.

#### Scenario: Extract C2PA data
- **WHEN** a media file with C2PA metadata is submitted
- **THEN** the system SHALL extract signature, certificate chain, public key, and claims

#### Scenario: Extract for ZKVM
- **WHEN** C2PA data is extracted
- **THEN** the system SHALL pass the data to ZK Proof Layer for in-ZKVM C2PA verification

#### Scenario: Missing metadata
- **WHEN** a media file without C2PA provenance metadata is submitted
- **THEN** the system SHALL indicate no C2PA data is present

### Requirement: Original Image Extraction
The system SHALL extract the original image data from signed media files.

#### Scenario: Extract original
- **WHEN** a signed media file passes verification
- **THEN** the system SHALL provide the original image bytes for downstream processing

### Requirement: Metadata Extraction
The system SHALL extract and expose provenance metadata.

#### Scenario: Extract device info
- **WHEN** media with device capture metadata is verified
- **THEN** the system SHALL expose device model, capture time, and other C2PA claims
