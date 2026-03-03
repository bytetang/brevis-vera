## ADDED Requirements

### Requirement: ZKVM guest SHALL re-execute crop operation on raw pixels
The ZKVM guest program SHALL accept raw RGBA pixel data as a private witness for each crop operation, extract the sub-rectangle defined by the crop parameters, and verify that the SHA-256 hash of the extracted pixels matches the claimed output hash.

#### Scenario: Valid crop re-execution
- **WHEN** the guest receives raw RGBA pixels (width=100, height=80), crop parameters (x=10, y=20, w=50, h=40), and the host claims input_hash and output_hash
- **THEN** the guest computes SHA-256 of the raw pixels, constrains it to equal input_hash, extracts the 50×40 sub-rectangle starting at (10,20), computes SHA-256 of the extracted pixels, and constrains it to equal output_hash

#### Scenario: Input hash mismatch
- **WHEN** the guest computes SHA-256 of the provided raw pixels and it does not match the claimed input_hash
- **THEN** the proof SHALL fail (editing_verified = 0)

#### Scenario: Output hash mismatch after re-execution
- **WHEN** the guest re-executes the crop and computes SHA-256 of the cropped pixels, but it does not match the claimed output_hash
- **THEN** the proof SHALL fail (editing_verified = 0)

### Requirement: ZKVM guest SHALL validate crop bounds
The ZKVM guest program SHALL verify that the crop region falls within the source image dimensions before attempting pixel extraction.

#### Scenario: Crop region exceeds image width
- **WHEN** crop parameters specify x + width > source image width
- **THEN** the proof SHALL fail (editing_verified = 0)

#### Scenario: Crop region exceeds image height
- **WHEN** crop parameters specify y + height > source image height
- **THEN** the proof SHALL fail (editing_verified = 0)

#### Scenario: Zero-dimension crop
- **WHEN** crop parameters specify width = 0 or height = 0
- **THEN** the proof SHALL fail (editing_verified = 0)

### Requirement: ImageWitness type SHALL carry raw pixel data
The shared library (`zk-guest/lib`) SHALL define an `ImageWitness` struct containing raw RGBA pixel bytes, width, and height. The `CircuitInput` SHALL include a `Vec<ImageWitness>` with one entry per editing operation.

#### Scenario: ImageWitness serialization round-trip
- **WHEN** an `ImageWitness` with 10×10 RGBA data (400 bytes) is serialized and deserialized
- **THEN** the deserialized struct SHALL have identical pixels, width, and height

#### Scenario: ImageWitness count matches editing records
- **WHEN** `CircuitInput` contains N editing records
- **THEN** `image_witnesses` SHALL contain exactly N entries, one per operation in order

### Requirement: Host prover SHALL pass raw pixels to ZKVM for crop operations
The `PicoProver` SHALL decode the image to raw RGBA pixels and include them in the `CircuitInput.image_witnesses` before writing to the ZKVM stdin.

#### Scenario: PicoProver prepares crop witness
- **WHEN** a crop proof is requested with image bytes
- **THEN** the prover SHALL decode the image to RGBA, include the raw pixel data as an `ImageWitness`, and write it to the ZKVM guest stdin

#### Scenario: Simulated prover with raw pixels
- **WHEN** the simulated prover receives editing input with raw pixel data present
- **THEN** it SHALL perform the same re-execution verification (hash input, crop, hash output, compare)

#### Scenario: Simulated prover without raw pixels (backward compatibility)
- **WHEN** the simulated prover receives editing input without raw pixel data
- **THEN** it SHALL fall back to parameter-only validation (current behavior)

### Requirement: Editor SHALL compute hashes over raw RGBA pixels for crop
The editor's crop function SHALL compute `original_image_hash` and `edited_image_hash` over raw RGBA pixel data (not PNG/JPEG encoded bytes), and SHALL include source image dimensions in the editing record parameters.

#### Scenario: Crop record includes raw-pixel hashes
- **WHEN** a crop operation is performed
- **THEN** the `EditingRecord` SHALL contain `original_image_hash` = SHA-256 of raw input RGBA pixels, `edited_image_hash` = SHA-256 of raw output RGBA pixels

#### Scenario: Crop record includes source dimensions
- **WHEN** a crop operation is performed on a 100×80 image
- **THEN** the `EditingRecord.parameters` SHALL include `source_width: 100` and `source_height: 80`

### Requirement: Crop pixel extraction SHALL use row-major RGBA layout
The crop re-execution inside the ZKVM SHALL interpret pixel data as row-major RGBA (4 bytes per pixel), and extract the sub-rectangle by copying the appropriate byte ranges from each row.

#### Scenario: Correct pixel extraction geometry
- **WHEN** source image is 100×80 RGBA (32000 bytes), crop is (x=10, y=20, w=50, h=40)
- **THEN** for each row r in [20..60], the guest extracts bytes [r*100*4 + 10*4 .. r*100*4 + 60*4] and concatenates them to form the cropped output (50×40×4 = 8000 bytes)
