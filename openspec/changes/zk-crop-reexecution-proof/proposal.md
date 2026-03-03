## Why

The current crop verification in both the host-side circuit (`src/zk/circuits.rs::verify_crop`) and the ZKVM guest (`zk-guest/app/src/main.rs::verify_operation_params`) only validates parameter format (non-zero dimensions, coordinates present) and hash chain continuity (input ≠ output, chain links match). This is fundamentally insufficient — an attacker can fabricate any pair of hashes with valid-looking crop parameters and the proof will pass. The ZK proof currently proves nothing about whether the output image was actually derived from the input image via the claimed crop operation.

A correct ZK proof of crop must **re-execute** the crop operation inside the ZKVM: take the original image pixels as a private witness, extract the sub-region, hash the result, and constrain that hash to equal the claimed output hash. Only then does the proof guarantee that "image B is the crop of image A at coordinates (x, y, w, h)".

## What Changes

- **Pass raw image bytes into the ZKVM as private witness** for each crop operation, instead of only hashes and parameters
- **Implement pixel-level crop re-execution in the guest program** — extract the sub-rectangle from raw pixel data (no image decoding; pre-decoded RGBA bytes)
- **Compute SHA-256 of both input and output inside the ZKVM** and constrain them to match the claimed hashes, using Pico's SHA-256 precompile acceleration
- **Update host-side prover** to serialize raw pixel data into the ZKVM stdin alongside crop parameters
- **Keep the host-side `verify_crop` in `circuits.rs`** as a fast pre-flight check (simulated prover), but add clear documentation that it is NOT a substitute for in-ZKVM re-execution
- **Scope: crop only** — resize and rotate re-execution are deferred (resize requires Lanczos3 in ZKVM which is significantly more complex)

## Capabilities

### New Capabilities
- `zk-crop-reexecution`: In-ZKVM crop re-execution with SHA-256 hash verification — proves output image is genuinely derived from input image via the declared crop parameters

### Modified Capabilities

## Impact

- **`zk-guest/lib/src/lib.rs`** — `EditingRecordData` and `CircuitInput` types need a field for raw pixel data (or a separate witness structure)
- **`zk-guest/app/src/main.rs`** — guest `verify_editing` / `verify_operation_params` must perform actual pixel extraction and SHA-256
- **`src/zk/prover.rs`** — `PicoProver` must pass raw image bytes to the guest stdin; `SimulatedProver` should also run the re-execution path for consistency
- **`src/zk/circuits.rs`** — `verify_crop` should be enhanced with the re-execution logic for the simulated prover path
- **`src/zk/types.rs`** — `EditingRecordInput` needs a field to carry raw pixel data
- **`src/editor/operations.rs`** — crop function should output raw RGBA pixels (or they should be extracted before proof generation) so the prover can pass them in
- **Performance** — proof generation will be slower due to hashing full image data in ZKVM; image size limits may need to be considered
- **Memory** — ZKVM guest memory usage increases proportionally to image size
