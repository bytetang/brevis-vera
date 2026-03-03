## 1. Shared Types (zk-guest/lib)

- [x] 1.1 Add `ImageWitness` struct to `zk-guest/lib/src/lib.rs` with `pixels: Vec<u8>`, `width: u32`, `height: u32` and derive Serialize/Deserialize
- [x] 1.2 Add `image_witnesses: Vec<ImageWitness>` field to `CircuitInput`
- [x] 1.3 Add serialization round-trip test for `ImageWitness` and updated `CircuitInput`

## 2. Editor Layer — Raw Pixel Hashing

- [x] 2.1 Update `crop()` in `src/editor/operations.rs` to compute `original_image_hash` and `edited_image_hash` over raw RGBA pixel data instead of PNG-encoded bytes
- [x] 2.2 Add `source_width` and `source_height` to crop `EditingRecord.parameters`
- [x] 2.3 Add helper function to extract raw RGBA pixels from a `PhotonImage` (or use existing `.get_raw_pixels()`)
- [x] 2.4 Update editor tests to verify hashes are computed over raw pixels and source dimensions are present in crop records

## 3. ZKVM Guest — Crop Re-execution

- [x] 3.1 Implement `crop_pixels(pixels: &[u8], img_w: u32, img_h: u32, x: u32, y: u32, w: u32, h: u32) -> Vec<u8>` helper in the guest program that extracts a sub-rectangle from row-major RGBA data
- [x] 3.2 Update `verify_editing()` in `zk-guest/app/src/main.rs` to accept `image_witnesses` and, for crop operations, call `crop_pixels` + SHA-256 to verify input/output hashes
- [x] 3.3 Add bounds checking inside the guest: fail proof if `x + w > img_w` or `y + h > img_h` or `w == 0` or `h == 0`
- [x] 3.4 Verify `sha256(raw_pixels) == input_hash` before crop, and `sha256(cropped_pixels) == output_hash` after crop
- [x] 3.5 For non-crop operations (resize, rotate), continue using parameter-only validation with no witness required

## 4. Host-side Circuit (Simulated Prover Path)

- [x] 4.1 Add optional `raw_pixels: Option<Vec<u8>>`, `pixel_width: Option<u32>`, `pixel_height: Option<u32>` fields to `EditingRecordInput` in `src/zk/types.rs`
- [x] 4.2 Update `verify_crop()` in `src/zk/circuits.rs` to perform re-execution when raw pixels are provided: SHA-256 input, extract sub-rect, SHA-256 output, compare hashes
- [x] 4.3 Keep parameter-only validation as fallback when `raw_pixels` is `None` (backward compat for simulated prover)
- [x] 4.4 Add `crop_pixels_raw()` helper in `src/zk/circuits.rs` matching the guest's implementation

## 5. Prover Integration

- [x] 5.1 Update `SimulatedProver` in `src/zk/prover.rs` to optionally populate `raw_pixels` in `EditingRecordInput` when image data is available
- [x] 5.2 Update `PicoProver` in `src/zk/prover.rs` to decode images to raw RGBA pixels and populate `CircuitInput.image_witnesses` before writing to ZKVM stdin
- [x] 5.3 Ensure `PicoProver` fails with a clear error if raw pixel data is missing for a crop operation

## 6. Testing

- [x] 6.1 Add unit test in `src/zk/circuits.rs`: valid crop re-execution with real pixel data produces valid result
- [x] 6.2 Add unit test: tampered output hash (wrong cropped pixels) fails verification
- [x] 6.3 Add unit test: tampered input hash (wrong source pixels) fails verification
- [x] 6.4 Add unit test: out-of-bounds crop parameters fail verification
- [x] 6.5 Add integration test: end-to-end crop → prove → verify with simulated prover using raw pixels
- [x] 6.6 Build guest program (`cargo pico build`) and verify ELF compiles with new witness types
