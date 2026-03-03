## Context

Currently, the Brevis Vera ZK proof for crop operations only validates **parameter format** and **hash chain continuity**. Both the host-side simulated circuit (`src/zk/circuits.rs::verify_crop`) and the ZKVM guest (`zk-guest/app/src/main.rs::verify_operation_params`) check that (width > 0, height > 0, coordinates present, input_hash ≠ output_hash). This gives zero cryptographic assurance that the output image was actually produced by cropping the input image.

The editor layer (`src/editor/operations.rs::crop`) correctly computes SHA-256 hashes before and after the crop, but this honest-prover behavior is never enforced inside the ZK circuit. An adversary can submit arbitrary hashes with plausible parameters and obtain a valid proof.

The Pico ZKVM guest already has SHA-256 precompile acceleration via patched `sha2` crate, making in-ZKVM hashing feasible. The guest program currently receives only hashes and parameters — it needs raw pixel data to perform re-execution.

## Goals / Non-Goals

**Goals:**
- Prove that output image B is the genuine crop of input image A at the declared coordinates, with cryptographic certainty
- Re-execute the crop operation inside the ZKVM: extract sub-rectangle from raw pixels, SHA-256 the input and output, constrain both hashes
- Keep the proof zero-knowledge: verifier learns only that "a crop was applied" and the image hashes, not the crop coordinates or pixel data
- Scope to crop operations only; other operations continue with parameter-only validation

**Non-Goals:**
- Re-executing resize operations in ZKVM (Lanczos3 interpolation is too complex for this change)
- Re-executing rotate operations in ZKVM (deferred — simpler than resize but still out of scope)
- Supporting compressed image formats (JPEG/PNG) inside the ZKVM — images are passed as pre-decoded raw RGBA pixels
- On-chain verifier contract changes
- Optimizing ZKVM memory for large images (>4K resolution) — out of scope, can be addressed later

## Decisions

### 1. Raw RGBA pixel representation

**Decision**: Pass pre-decoded RGBA pixel data (Vec<u8>, 4 bytes per pixel) into the ZKVM, not encoded PNG/JPEG.

**Rationale**: Image format decoding (PNG decompression, JPEG DCT) inside a ZKVM is prohibitively expensive and adds massive circuit complexity. The host can decode the image before passing it in. The hash is computed over the same raw bytes, so the proof still binds to the actual pixel content.

**Alternative considered**: Pass PNG bytes and decode inside ZKVM — rejected due to ~100x overhead of PNG decompression in RISC-V.

### 2. Hash-then-crop workflow

**Decision**: The ZKVM guest will:
1. Receive `(raw_pixels, width, height, crop_x, crop_y, crop_w, crop_h)` as private input
2. Compute `sha256(raw_pixels)` → constrain == `input_hash`
3. Extract sub-rectangle pixels: for each row in `[crop_y..crop_y+crop_h]`, copy `[crop_x*4..(crop_x+crop_w)*4]`
4. Compute `sha256(cropped_pixels)` → constrain == `output_hash`

**Rationale**: This is the minimal set of operations needed to prove the crop relationship. SHA-256 uses Pico's precompile so it's fast. The pixel extraction is pure memory copy — one of the cheapest operations in a ZKVM.

**Alternative considered**: Hash the image in a Merkle tree and prove sub-region inclusion — more complex, marginal performance benefit for typical image sizes.

### 3. Dual data path: raw pixels for ZKVM, hash-only for simulated prover

**Decision**: 
- `EditingRecordInput` gains an optional `raw_pixels: Option<Vec<u8>>` field (+ `pixel_width`, `pixel_height`)
- The `PicoProver` requires raw pixels and passes them to the guest
- The `SimulatedProver` can optionally perform re-execution if raw pixels are provided, but falls back to parameter-only validation (current behavior) if not
- The ZKVM guest `CircuitInput` gains a `Vec<ImageWitness>` field carrying raw pixel data for each operation

**Rationale**: Keeps backward compatibility for simulated prover tests while enabling the full re-execution path for Pico proofs.

### 4. Image representation in guest types

**Decision**: Add a new `ImageWitness` struct to `zk-guest/lib`:
```rust
pub struct ImageWitness {
    pub pixels: Vec<u8>,    // RGBA raw bytes
    pub width: u32,
    pub height: u32,
}
```

The `CircuitInput` gains `image_witnesses: Vec<ImageWitness>` — one per editing operation, containing the **input** image for that operation. The output is derived by re-executing the operation inside the ZKVM.

**Rationale**: Separates pixel data from parameters, keeps `EditingRecordData` lean, and allows the witness list to be independently sized.

### 5. Bounds checking inside ZKVM

**Decision**: The guest program will validate `crop_x + crop_w <= width` and `crop_y + crop_h <= height` inside the circuit, and fail the proof if violated.

**Rationale**: Even though the host-side editor already checks bounds, the ZKVM must independently verify to prevent a malicious host from submitting out-of-bounds parameters.

## Risks / Trade-offs

**[Memory usage scales with image size]** → For a 4K image (3840×2160×4 = 33MB), the ZKVM guest needs ~33MB for input + ~cropped region for output. Pico ZKVM's memory limit may be hit. **Mitigation**: Document maximum supported image size; for this change, test with images up to 1920×1080.

**[Proof generation time increases]** → SHA-256 over a full image inside ZKVM adds significant proving time. **Mitigation**: Pico's SHA-256 precompile makes this ~1000x faster than naive implementation. Benchmark and document expected times.

**[Raw pixel hash differs from encoded file hash]** → The editor currently hashes PNG-encoded bytes. If we hash raw RGBA pixels in the ZKVM, the hashes won't match. **Mitigation**: The editor must be updated to hash raw pixel data (pre-encoding) for the crop proof path, or the ZKVM must also receive the encoding function. We choose the former — hash raw pixels consistently.

**[Breaking change in hash computation]** → Switching from PNG-byte hashes to raw-pixel hashes changes `EditingRecord.original_image_hash` and `edited_image_hash` values. **Mitigation**: This is an internal change; no external API contract exists yet. Update all hash computation to use raw pixels consistently. Consider supporting both modes during transition.

**[Only crop is re-executed]** → Resize and rotate still use parameter-only validation, which is weaker. **Mitigation**: Clearly document which operations have full re-execution proofs vs. parameter-only validation. Plan follow-up changes for rotate (simpler) then resize (complex).
