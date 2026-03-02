## ADDED Requirements

### Requirement: Crop API Endpoint
The system SHALL provide a REST API endpoint for cropping images.

#### Scenario: Successful crop request
- **WHEN** `POST /api/v1/edit/crop` is called with valid image_id and coordinates
- **THEN** the system SHALL return the cropped image and editing record

#### Scenario: Crop with invalid bounds
- **WHEN** crop coordinates exceed image dimensions
- **THEN** the system SHALL return HTTP 400 error

### Requirement: Resize API Endpoint
The system SHALL provide a REST API endpoint for resizing images.

#### Scenario: Successful resize request
- **WHEN** `POST /api/v1/edit/resize` is called with valid image_id and dimensions
- **THEN** the system SHALL return the resized image and editing record

### Requirement: Rotate API Endpoint
The system SHALL provide a REST API endpoint for rotating images.

#### Scenario: Successful rotate request
- **WHEN** `POST /api/v1/edit/rotate` is called with valid image_id and angle
- **THEN** the system SHALL return the rotated image and editing record

### Requirement: Editing Record Generation
The system SHALL generate an editing record for each operation.

#### Scenario: Record crop operation
- **WHEN** a crop operation is performed
- **THEN** the system SHALL create a record with operation type "crop" and parameters `{x, y, width, height}`

#### Scenario: Record resize operation
- **WHEN** a resize operation is performed
- **THEN** the system SHALL create a record with operation type "resize" and parameters `{original_width, original_height, new_width, new_height}`

#### Scenario: Record rotate operation
- **WHEN** a rotate operation is performed
- **THEN** the system SHALL create a record with operation type "rotate" and parameters `{angle}`

### Requirement: Editing Record Format
The system SHALL return a standardized editing record format.

#### Scenario: Return editing record
- **WHEN** an editing operation completes successfully
- **THEN** the system SHALL return JSON:
```json
{
  "operation": "crop|resize|rotate",
  "parameters": {...},
  "original_image_hash": "...",
  "edited_image_hash": "...",
  "timestamp": "ISO8601"
}
```
