# AOT Optimized Emulator Configuration Guide

This guide explains how to enable and configure the **AOT (Ahead-of-Time) Optimized Emulator** in projects using pico. Based on the analysis of both the pico repository examples and the current brevis-vera project.

---

## Table of Contents

1. [Overview](#overview)
2. [Pre-compiled Chunks (Production)](#pre-compiled-chunks-production)
3. [Brevis-Vera Implementation](#brevis-vera-implementation)
4. [Feature Flags Configuration](#feature-flags-configuration)
5. [Troubleshooting](#troubleshooting)
6. [Environment Variables](#environment-variables)
7. [Additional Resources](#additional-resources)

---

## Overview

The pico project supports two execution modes:

| Mode | Description |
|------|-------------|
| **JIT (Just-In-Time)** | Interprets RISC-V instructions at runtime |
| **AOT (Ahead-of-Time)** | Pre-compiled, optimized execution |

The AOT mode provides significantly better performance by pre-compiling the RISC-V program into optimized chunks before execution.

---

## Pre-compiled Chunks (Production)

This approach is used in brevis-vera for maximum performance. It involves pre-compiling the RISC-V program into multiple chunks.

### Architecture Overview

```
brevis-vera-guest (Rust RISC-V program)
    ↓ Compile to ELF
    cargo pico build
brevis-vera/zk-guest/app/elf/riscv32im-pico-zkvm-elf
    ↓ AOT Compile
    aot-chunks/ (43 pre-compiled chunks)
```

### Step 1: Build AOT Codegen Tool

Build the pico repository:

```bash
cd /path/to/pico
cargo build -p pico-aot-codegen --release
```

The binary will be at `target/release/generate_crates`.

### Step 2: Generate AOT Chunks

```bash
# From your project root
/path/to/pico/target/release/generate_crates \
    zk-guest/app/elf/riscv32im-pico-zkvm-elf \
    zk-guest/app/aot-chunks
```

This generates:
- `aot-chunks/Cargo.toml` - Main dispatch crate
- `aot-chunks/chunks/chunk_000` to `chunk_042` - 43 chunk crates

### Step 3: Configure Chunk Crates

Update each chunk's `Cargo.toml` to use git dependencies instead of local paths:

```bash
cd zk-guest/app/aot-chunks/chunks
for i in chunk_*; do
    sed -i '' 's|pico-aot-runtime = { path = ".*" }|pico-aot-runtime = { git = "https://github.com/brevis-network/pico" }|' $i/Cargo.toml
done
```

### Step 4: Configure Main Dispatch Crate

Update `aot-chunks/Cargo.toml`:

```toml
[package]
name = "brevis-vera-aot-dispatch"
version = "0.1.0"
edition = "2021"

[package.metadata.infinite]
aot = true

[features]
default = []
pico = []

[dependencies]
pico-aot-runtime = { git = "https://github.com/brevis-network/pico" }
pico-aot-chunk-000 = { path = "chunks/chunk_000" }
# ... all other chunks (chunk_001 to chunk_042)
```

### Step 5: Add to Workspace

Add to your root `Cargo.toml`:

```toml
[workspace]
members = [".", "zk-guest/lib", "zk-guest/app", "zk-guest/app/aot-chunks"]
resolver = "2"
```

Add as dependency:

```toml
brevis-vera-aot-dispatch = { path = "zk-guest/app/aot-chunks", default-features = false }
```

### Step 6: Create run_aot Function

The generated `aot-chunks/src/lib.rs` provides:

```rust
pub use pico_aot_runtime::AotEmulatorCore;

pub fn run_aot(emu: &mut AotEmulatorCore) -> Result<(), String> {
    // AOT dispatch logic
}
```

---

## Brevis-Vera Implementation

This section documents how brevis-vera implements AOT.

### Project Structure

```
brevis-vera/
├── Cargo.toml                      # Workspace root with pico-aot feature
├── zk-guest/
│   ├── lib/                        # Shared types
│   └── app/
│       ├── elf/
│       │   └── riscv32im-pico-zkvm-elf
│       └── aot-chunks/            # Generated AOT chunks
│           ├── Cargo.toml
│           ├── src/lib.rs
│           └── chunks/
│               ├── chunk_000/
│               ├── chunk_001/
│               └── ...
```

### Key Files

| File | Description |
|------|-------------|
| `zk-guest/app/aot-chunks/src/lib.rs` | AOT dispatch with `run_aot` function |
| `Cargo.toml` | Workspace with `brevis-vera-aot-dispatch` dependency |
| `src/zk/prover.rs` | PicoProver with AOT/JIT mode support |

---

## Feature Flags Configuration

### Cargo.toml Configuration

```toml
[features]
default = ["pico-aot"]  # AOT mode is default
pico = ["dep:pico-sdk", "dep:alloy-sol-types", "dep:bincode", "dep:brevis-vera-zk-lib", "dep:brevis-vera-aot-dispatch", "dep:pico-vm"]
pico-aot = ["pico", "dep:pico-vm"]

[dependencies]
pico-vm = { git = "https://github.com/brevis-network/pico", optional = true }
brevis-vera-aot-dispatch = { path = "zk-guest/app/aot-chunks", default-features = false, optional = true }
```

### API Configuration (src/zk/api.rs)

The API handler uses conditional compilation to support both modes:

```rust
#[cfg(any(feature = "pico", feature = "pico-aot"))]
use crate::zk::prover::PicoProver;

#[cfg(any(feature = "pico", feature = "pico-aot"))]
const DEFAULT_ELF_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/zk-guest/app/elf/riscv32im-pico-zkvm-elf");

// Create prover based on feature
#[cfg(any(feature = "pico", feature = "pico-aot"))]
let prover: Box<dyn ZkProver> = match PicoProver::new(DEFAULT_ELF_PATH) {
    Ok(p) => {
        #[cfg(feature = "pico-aot")]
        tracing::info!("Using PicoProver (AOT mode)");
        #[cfg(not(feature = "pico-aot"))]
        tracing::info!("Using PicoProver (JIT mode)");
        Box::new(p)
    }
    // ...
};
```

### Prover Implementation (src/zk/prover.rs)

The PicoProver automatically selects AOT or JIT based on feature:

```rust
fn run_pico_prover(&self, combined: &CombinedProofInput) -> Result<ZkProof, ZkError> {
    #[cfg(feature = "pico-aot")]
    {
        self.run_pico_prover_aot(combined)
    }

    #[cfg(not(feature = "pico-aot"))]
    {
        self.run_pico_prover_jit(combined)
    }
}
```

### Build Commands

```bash
# AOT mode (default) - faster execution
cargo run --release
cargo test

# JIT mode - slower but easier debugging
cargo run --release --features pico
cargo test --features pico

# No ZK (SimulatedProver)
cargo run --release --no-default-features
cargo test --no-default-features
```

---

## Troubleshooting

### Error: Multiple workspace roots

If you see "multiple workspace roots found", ensure the AOT crate has its own `[workspace]` table:

```toml
[workspace]
```

### Error: Missing features

If you see "package does not contain this feature", add empty features to your AOT crate:

```toml
[features]
default = []
pico = []
```

### Error: Compilation errors with chunks

If chunk compilation fails, ensure all chunks use the same `pico-aot-runtime` source:

```bash
# Update all chunks
cd aot-chunks/chunks
for i in chunk_*; do
    sed -i '' 's|pico-aot-runtime = { path = ".*" }|pico-aot-runtime = { git = "https://github.com/brevis-network/pico" }|' $i/Cargo.toml
done
```

### Error: prover_type mismatch in tests

When running tests, the prover_type returned differs between modes:
- JIT mode: `"pico"`
- AOT mode: `"pico-aot"`

Update test assertions to accept both:

```rust
assert!(
    proof.metadata.prover_type == "pico" || proof.metadata.prover_type == "pico-aot",
    "prover_type should be 'pico' or 'pico-aot', got {}",
    proof.metadata.prover_type
);
```

### Error: AOT chunks out of sync with ELF

When modifying the guest program, rebuild both the ELF and AOT chunks:

```bash
# 1. Rebuild guest ELF
cd zk-guest/app && cargo pico build

# 2. Regenerate AOT chunks
/path/to/pico/target/release/generate_crates \
    zk-guest/app/elf/riscv32im-pico-zkvm-elf \
    zk-guest/app/aot-chunks

# 3. Rebuild the project
cargo build
```

---

## Environment Variables

The AOT emulator respects several environment variables:

```bash
# STARK proof configuration
FRI_QUERIES=84           # Number of FRI queries (default: 84)
LOG_BLOWUP=1             # FRI extension factor (default: 1)
PROOF_OF_WORK_BITS=16    # Proof of work bits (default: 16)
```

---

## Additional Resources

- [pico repository](https://github.com/brevis-network/pico)
- [pico reth AOT example](https://github.com/brevis-network/pico/tree/main/examples/reth/aot)
- [AOT Runtime source code](./aot-runtime/src/)
