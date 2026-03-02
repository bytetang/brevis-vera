## ADDED Requirements

### Requirement: API Integration
The frontend SHALL integrate with the backend APIs for provenance, editing, and proof generation.

#### Scenario: Call Capture Layer API
- **WHEN** user uploads an image
- **THEN** the system SHALL call `POST /api/v1/provenance/extract` to extract C2PA metadata

#### Scenario: Call Editing Layer APIs
- **WHEN** user performs an editing operation
- **THEN** the system SHALL call the appropriate API:
  - `POST /api/v1/edit/crop` for crop operations
  - `POST /api/v1/edit/resize` for resize operations
  - `POST /api/v1/edit/rotate` for rotate operations

#### Scenario: Call ZK Proof API
- **WHEN** user clicks "Generate Proof"
- **THEN** the system SHALL call `POST /api/v1/zk/prove` with editing records

### Requirement: Image Upload
The system SHALL allow users to upload images via drag-and-drop or file selection.

#### Scenario: Upload valid image
- **WHEN** user drags and drops a JPEG or PNG file
- **THEN** the system SHALL display image preview and extract C2PA metadata

#### Scenario: Upload invalid file type
- **WHEN** user uploads a non-image file
- **THEN** the system SHALL display an error message

### Requirement: C2PA Provenance Display
The system SHALL display C2PA provenance information extracted from the uploaded image.

#### Scenario: Display device info
- **WHEN** image has valid C2PA metadata
- **THEN** the system SHALL display camera model, manufacturer, and capture time

#### Scenario: Display signature info
- **WHEN** image has valid C2PA metadata
- **THEN** the system SHALL display signature algorithm (e.g., ES256)

#### Scenario: No provenance data
- **WHEN** uploaded image has no C2PA metadata
- **THEN** the system SHALL indicate "No C2PA data found"

### Requirement: Image Editing
The system SHALL provide interactive tools for editing images.

#### Scenario: Crop image
- **WHEN** user selects crop area and confirms
- **THEN** the system SHALL display cropped preview and update editing record

#### Scenario: Resize image
- **WHEN** user enters new dimensions and confirms
- **THEN** the system SHALL display resized preview and update editing record

#### Scenario: Rotate image
- **WHEN** user clicks rotate button (90°/180°/270°)
- **THEN** the system SHALL display rotated preview and update editing record

### Requirement: ZK Proof Generation
The system SHALL generate a ZK proof for the editing operations.

#### Scenario: Generate proof successfully
- **WHEN** user clicks "Generate Proof" button
- **THEN** the system SHALL display loading state, then show proof details

#### Scenario: Proof generation fails
- **WHEN** proof generation encounters an error
- **THEN** the system SHALL display error message

### Requirement: Verification Result Display
The system SHALL display the verification results from the ZK proof.

#### Scenario: Display verification results
- **WHEN** proof generation completes successfully
- **THEN** the system SHALL display:
```json
{
  "c2pa_verified": true | false,
  "editing_verified": true | false,
  "operations_applied": ["Crop", "Resize", "Rotate"],
  "proof_size_bytes": number
}
```

### Requirement: Download
The system SHALL allow users to download edited image and ZK proof.

#### Scenario: Download edited image
- **WHEN** user clicks "Download Image" button
- **THEN** the system SHALL download the edited image file

#### Scenario: Download ZK Proof
- **WHEN** user clicks "Download Proof" button
- **THEN** the system SHALL download the proof file
