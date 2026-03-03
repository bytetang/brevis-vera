# Brevis Vera

End-to-end digital media authenticity attestation system that bridges hardware-level provenance (C2PA signatures) and software-level editing verification using Zero-Knowledge Proofs (ZK Proofs) powered by [Pico ZKVM](https://github.com/brevis-network/pico).

## Overview

In an era of AI-generated and manipulated media, Brevis Vera enables:

- **Content Authenticity** — prove media originated from a real device via C2PA metadata
- **Editing Transparency** — demonstrate that specific edits (crop, resize, rotate) were applied
- **Privacy-Preserving Verification** — generate ZK proofs that verify authenticity without revealing sensitive details (original image, exact parameters, signer identity)

### Architecture

```
Media File ─→ Provenance Layer ─→ Editing Layer ─→ ZK Proof Layer ─→ Verifiable Proof
                  │                    │                  │
                  ├─ C2PA extraction   ├─ Crop/Resize/    ├─ Pico ZKVM (STARK)
                  └─ Format detection  │  Rotate          └─ SimulatedProver (dev)
                                       └─ Hash chaining
```

### Project Structure

```
brevis-vera/
├── src/                    # Rust backend
│   ├── main.rs             # Axum HTTP server (port 8080)
│   ├── provenance/         # C2PA metadata extraction & parsing
│   ├── editor/             # Image editing (crop, resize, rotate)
│   └── zk/                 # ZK proof generation & verification
├── zk-guest/               # Pico ZKVM guest program
│   ├── app/                # RISC-V guest (compiled to ELF)
│   └── lib/                # Shared types between host & guest
├── frontend/               # React + TypeScript UI
│   └── src/
│       ├── components/     # ImageUploader, ImageEditor, etc.
│       ├── services/       # API client
│       └── types/          # TypeScript type definitions
├── certs/                  # Test certificates for C2PA signing
├── imgs/                   # Test images with C2PA metadata
└── tests/                  # Integration & E2E tests
```

## Prerequisites

- **Rust** (edition 2024) with Cargo
- **Node.js** (v18+) and npm
- **Pico ZKVM toolchain** (for real proof generation)

## Getting Started

### 1. Build the ZK Guest Program

The guest program runs inside the Pico ZKVM to generate real STARK proofs:

```bash
cd zk-guest/app
cargo pico build
```

This produces the ELF binary at `zk-guest/app/elf/riscv32im-pico-zkvm-elf`.

### 2. Start the Backend Server

```bash
# With real ZK proofs (PicoProver, default)
cargo run --release

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

## Testing

### Unit & Integration Tests

```bash
# Run all tests (uses SimulatedProver by default for speed)
cargo test

# Run all tests including Pico ZKVM tests (slower, generates real proofs)
cargo test --features pico

# Run a specific test suite
cargo test --test integration          # Integration tests
cargo test --test editing_api          # Editing API tests
cargo test --test zk_proof             # ZK proof tests (simulated)
cargo test --test pico_vm_proof --features pico  # Pico ZKVM E2E tests (slow)
```

### Test Suites

| File | Description | Speed |
|------|-------------|-------|
| `tests/integration.rs` | End-to-end provenance + editing pipeline | Fast |
| `tests/editing_api.rs` | Editing API (crop, resize, rotate) | Fast |
| `tests/zk_proof.rs` | ZK proof generation with SimulatedProver | Fast |
| `tests/pico_vm_proof.rs` | Real ZKVM proof generation with PicoProver | Slow (~30s per test) |

### Benchmark: Run Specific Edit Operation Tests

```bash
# Run all Pico edit operation tests (release mode for accurate timing)
cargo test --features pico --test pico_vm_proof --release -- --nocapture

# Run only crop tests
cargo test --features pico --test pico_vm_proof --release crop -- --nocapture

# Run only resize tests
cargo test --features pico --test pico_vm_proof --release resize -- --nocapture

# Run only rotate tests
cargo test --features pico --test pico_vm_proof --release rotate -- --nocapture
```

### Run Only Fast Tests

```bash
cargo test --no-default-features
```

### Frontend

```bash
cd frontend
npm run build    # Type-check + build
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `pico` | ✅ | Enable Pico ZKVM prover for real STARK/SNARK proof generation |

To disable the `pico` feature (dev mode with SimulatedProver only):

```bash
cargo run --release --no-default-features
cargo test --no-default-features
```

## Technology Stack

| Component | Technology |
|-----------|------------|
| Backend | Rust, Axum, Tokio |
| Image Processing | Photon, image crate |
| C2PA Parsing | c2pa crate |
| ZK Proofs | Pico ZKVM (STARK), ECDSA P-256 |
| Frontend | React 18, TypeScript, Ant Design, Vite |
| Hashing | SHA-256 |
