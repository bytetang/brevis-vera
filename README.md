# Brevis Vera

End-to-end digital media authenticity attestation system that bridges hardware-level provenance (C2PA signatures) and software-level editing verification using Zero-Knowledge Proofs (ZK Proofs) powered by [Pico ZKVM](https://github.com/brevis-network/pico).

## Overview

In an era of AI-generated and manipulated media, Brevis Vera enables:

- **Content Authenticity** — prove media originated from a real device via C2PA metadata
- **Editing Transparency** — demonstrate that specific edits (crop, resize, rotate) were applied
- **Privacy-Preserving Verification** — generate ZK proofs that verify authenticity without revealing sensitive details (original image, exact parameters, signer identity)

## Architecture

```
Media File ─→ Provenance Layer ─→ Editing Layer ─→ ZK Proof Layer ─→ Verifiable Proof
                  │                    │                  │
                  ├─ C2PA extraction   ├─ Crop/Resize/    ├─ Pico ZKVM (STARK)
                  └─ Format detection  │  Rotate          └─ SimulatedProver (dev)
                                       └─ Hash chaining
```

## What the ZKVM Proves

The Pico ZKVM generates STARK proofs that cryptographically verify:

### 1. C2PA Provenance Verification

| Check | Description |
|-------|-------------|
| **Structural Validation** | Verifies C2PA manifest has required fields (active_manifest, claim_generator) and recognized signing algorithm (ES256, ES384, PS256, etc.) |
| **ECDSA P-256 Signature** | Cryptographically verifies the ECDSA signature over the image hash using the provided public key |

### 2. Editing Operations Verification

The proof verifies that editing operations were actually performed on the image by **re-executing** them inside the ZKVM:

| Operation | Verification Method |
|-----------|---------------------|
| **Crop** | Extracts sub-rectangle from raw RGBA pixels, computes SHA-256, verifies output hash matches |
| **Rotate** | Re-executes 90°/180°/270° rotation via pixel permutation, verifies output hash |
| **Resize** | Re-executes nearest-neighbor interpolation with integer arithmetic, verifies output hash |

### 3. Hash Chain Verification

Each editing operation's output hash must equal the next operation's input hash, forming a cryptographic chain from original to edited image.

### Privacy Guarantees

The proof reveals **ONLY** these public values:

```
PublicValuesStruct {
    c2pa_verified: bool,      // Whether C2PA structure was valid
    ecdsa_verified: bool,     // Whether ECDSA signature verified
    editing_verified: bool,   // Whether editing chain verified
    original_image_hash: bytes32,  // SHA-256 of original image
    edited_image_hash: bytes32,    // SHA-256 of edited image
    num_operations: uint32,        // Number of editing operations
}
```

The proof does **NOT** reveal:
- C2PA signer identity or certificate details
- Exact crop coordinates, resize dimensions, or rotation angles
- Original image content or pixels
- ECDSA signature values or public key used

## Proof Flow

```
Host Application
      │
      ▼
┌─────────────────────────┐
│  1. Prepare Input       │
│  - CircuitInput         │
│  - C2PA data (optional) │
│  - Editing records      │
│  - Image witnesses      │
└─────────────────────────┘
      │
      ▼ (stdin)
┌─────────────────────────┐
│  2. ZKVM Execution      │
│  - Verify C2PA          │
│  - Verify ECDSA sig     │
│  - Re-execute edits     │
│  - Verify hash chain    │
└─────────────────────────┘
      │
      ▼ (commit_bytes)
┌─────────────────────────┐
│  3. Public Values       │
│  - Verification bools   │
│  - Image hashes         │
│  - Operation count      │
└─────────────────────────┘
      │
      ▼
   STARK Proof
```

## Getting Started

### Prerequisites

- **Rust** (edition 2024) with Cargo
- **Node.js** (v18+) and npm
- **Pico ZKVM toolchain** (for real proof generation)

### 1. Build the ZK Guest Program

```bash
cd zk-guest/app
cargo pico build
```

This produces the ELF binary at `zk-guest/app/elf/riscv32im-pico-zkvm-elf`.

### 2. Start the Backend Server

```bash
# With real ZK proofs (PicoProver AOT mode, default)
cargo run --release

# With real ZK proofs (PicoProver JIT mode)
cargo run --release --features pico

# Without ZK (SimulatedProver, faster for development)
cargo run --release --no-default-features
```

The server starts on `http://localhost:8080`.

### 3. Start the Frontend

```bash
cd frontend
npm install
npm run dev
```

The frontend starts on `http://localhost:3000`.

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/health` | Health check |
| `POST` | `/api/v1/provenance/extract` | Extract C2PA metadata from an image |
| `POST` | `/api/v1/edit/crop` | Crop an image |
| `POST` | `/api/v1/edit/resize` | Resize an image |
| `POST` | `/api/v1/edit/rotate` | Rotate an image (90°/180°/270°) |
| `POST` | `/api/v1/zk/prove` | Generate a ZK proof (c2pa / editing / combined) |

### Example: Extract Provenance

```bash
IMAGE_B64=$(base64 -i imgs/test_img_small.JPG)
curl -X POST http://localhost:8080/api/v1/provenance/extract \
  -H "Content-Type: application/json" \
  -d "{\"image\": \"$IMAGE_B64\"}"
```

## Project Structure

```
brevis-vera/
├── src/                    # Rust backend
│   ├── main.rs             # Axum HTTP server (port 8080)
│   ├── provenance/         # C2PA metadata extraction & parsing
│   ├── editor/             # Image editing (crop, resize, rotate)
│   └── zk/                 # ZK proof generation & verification
├── zk-guest/               # Pico ZKVM guest program
│   ├── app/                # RISC-V guest (compiled to ELF)
│   │   └── src/main.rs    # ZKVM proof logic (crop/rotate/resize/ECDSA)
│   └── lib/                # Shared types between host & guest
├── frontend/               # React + TypeScript UI
├── certs/                  # Test certificates for C2PA signing
├── imgs/                   # Test images with C2PA metadata
└── tests/                  # Integration & E2E tests
```

## Testing

### Run Tests

```bash
# Run all tests (uses PicoProver AOT mode by default)
cargo test

# Run all tests with JIT mode (slower)
cargo test --features pico

# Run all tests without ZK (SimulatedProver only)
cargo test --no-default-features
```

### Test Suites

| File | Description | Speed |
|------|-------------|-------|
| `tests/integration.rs` | End-to-end provenance + editing pipeline | Fast |
| `tests/editing_api.rs` | Editing API (crop, resize, rotate) | Fast |
| `tests/zk_proof.rs` | ZK proof generation with SimulatedProver | Fast |
| `tests/pico_vm_proof.rs` | Real ZKVM proof generation with PicoProver | Slow |

### Benchmark Specific Operations

```bash
# Run all Pico edit operation tests (release mode)
cargo test --features pico --test pico_vm_proof --release -- --nocapture

# Run only crop tests
cargo test --features pico --test pico_vm_proof --release crop -- --nocapture

# Run only resize tests
cargo test --features pico --test pico_vm_proof --release resize -- --nocapture

# Run only rotate tests
cargo test --features pico --test pico_vm_proof --release rotate -- --nocapture
```

### AOT (Ahead-of-Time) Mode

Pico supports two execution modes:

- **JIT (Just-In-Time)**: Interprets RISC-V instructions at runtime
- **AOT (Ahead-of-Time)**: Pre-compiled, optimized execution (faster, default)

```bash
# AOT mode (default)
cargo test test_pico_real_image_crop_reexecution -- --nocapture

# JIT mode
cargo test --features pico test_pico_real_image_crop_reexecution -- --nocapture
```

**Note:** When using AOT mode, rebuild the AOT chunks after modifying the guest program:

```bash
# 1. Rebuild the guest ELF
cd zk-guest/app && cargo pico build

# 2. Regenerate AOT chunks
/path/to/pico/target/release/generate_crates \
    zk-guest/app/elf/riscv32im-pico-zkvm-elf \
    zk-guest/app/aot-chunks
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `pico` | ❌ | Enable Pico ZKVM prover for real STARK/SNARK proof generation (JIT mode) |
| `pico-aot` | ✅ | Enable AOT (Ahead-of-Time) mode for faster execution |

## Technology Stack

| Component | Technology |
|-----------|------------|
| Backend | Rust, Axum, Tokio |
| Image Processing | Photon, image crate |
| C2PA Parsing | c2pa crate |
| ZK Proofs | Pico ZKVM (STARK), ECDSA P-256 |
| Frontend | React 18, TypeScript, Ant Design, Vite |
| Hashing | SHA-256 |

## FAQ

### Q: Why must ECDSA P-256 signature verification happen inside the ZKVM, not just on the host?

The purpose of signature verification is to prove that a camera (or other trusted device) actually produced the image — the C2PA standard embeds an ECDSA P-256 signature in the image metadata, signed by the device's private key.

If verification only ran on the host (outside the ZKVM), anyone could claim "I verified the signature and it passed" without cryptographic proof. The verifier would have to **trust the host**, which defeats the purpose of ZK proofs.

By running ECDSA verification **inside the ZKVM**, the resulting STARK proof cryptographically guarantees:

1. The signature was valid against a specific public key
2. The signature was computed over the exact `original_image_hash`
3. No one tampered with the verification logic

The verifier only needs to trust the math of the proof system, not the host. This is the core value proposition: **trustless, privacy-preserving verification**.

### Q: Why must compressed JPEG images be decoded to raw RGBA pixels for proof generation?

Two fundamental constraints make raw RGBA pixels necessary:

**1. Re-execution requires pixel access.** The ZKVM guest proves editing correctness by **re-executing** the operation (e.g., extracting a crop sub-rectangle, permuting pixels for rotation). This requires direct access to individual pixel values, which is impossible on compressed JPEG data.

**2. JPEG is non-deterministic.** Different JPEG encoder/decoder implementations (or even different versions of the same library) can produce slightly different pixel values from the same JPEG file. If the host decodes a JPEG with library A and the ZKVM guest decodes it with library B, their SHA-256 hashes won't match, breaking the verification chain.

Using raw RGBA pixels (4 bytes per pixel: R, G, B, Alpha) guarantees **bit-exact consistency** — the host and guest operate on identical data, producing identical hashes. The tradeoff is size: a 4320×2880 image is ~47MB in RGBA vs ~3MB in JPEG. This can be mitigated with lossless compression (e.g., zstd) for transport.

### Q: How does the ZKVM editing proof chain work to guarantee image authenticity?

The proof forms a **cryptographic hash chain** that links the original image to the final edited image through every operation:

```
Original Image                    Edited Image
     │                                 ▲
     ▼                                 │
  SHA-256(pixels₀) ──────────────────────────────────────────────
     = input_hash₀                                              │
     │                                                          │
     ▼                                                          │
  ┌──────────────┐    SHA-256(result₁)    ┌──────────────┐      │
  │  Operation 1 │ ──→ output_hash₁ ──→   │  Operation 2 │      │
  │  (e.g. crop) │     = input_hash₁      │ (e.g.rotate) │      │
  └──────────────┘                        └──────────────┘      │
                                            │                   │
                                            ▼                   │
                                      SHA-256(result₂)          │
                                        = output_hash₁          │
                                        = edited_image_hash ────┘
```

For each operation, the ZKVM guest performs these steps:

1. **Verify input**: Compute `SHA-256(witness_pixels)` and check it equals `input_hash` — proves the witness data is authentic
2. **Re-execute**: Apply the operation (crop/rotate/resize) on the witness pixels using deterministic algorithms (integer-only arithmetic, no floating point)
3. **Verify output**: Compute `SHA-256(result_pixels)` and check it equals `output_hash` — proves the operation was applied correctly
4. **Chain continuity**: Check that `output_hash[i] == input_hash[i+1]` — proves no tampering between operations
5. **Boundary check**: Verify `input_hash[0] == original_image_hash` and `output_hash[last] == edited_image_hash` — anchors the chain to the claimed original and final images

If **any** step fails — wrong pixels, modified parameters, skipped operations, or injected operations — the hash chain breaks and `editing_verified` is set to `false`. This guarantees that the edited image is the exact result of applying the claimed operations to the original image, with no hidden modifications.
