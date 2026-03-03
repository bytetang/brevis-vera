# AOT Optimized Emulator Configuration Guide

This guide explains how to enable and configure the **AOT (Ahead-of-Time) Optimized Emulator** in projects using pico. Based on the analysis of both the pico repository examples and the current brevis-vera-zk project.

---

## Table of Contents

1. [Overview](#overview)
2. [Two Ways to Enable AOT](#two-ways-to-enable-aot)
3. [Method 1: Feature Flag (Simple)](#method-1-feature-flag-simple)
4. [Method 2: Pre-compiled Chunks (Production)](#method-2-pre-compiled-chunks-production)
5. [Brevis-Vera Implementation](#brevis-vera-implementation)
6. [Comparison](#comparison)
7. [Migration Steps](#migration-steps)
8. [Troubleshooting](#troubleshooting)

---

## Overview

The pico project supports two execution modes:

| Mode | Description |
|------|-------------|
| **JIT (Just-In-Time)** | Interprets RISC-V instructions at runtime |
| **AOT (Ahead-of-Time)** | Pre-compiled, optimized execution |

The AOT mode provides significantly better performance by pre-compiling the RISC-V program into optimized chunks before execution.

---

## Two Ways to Enable AOT

### Method 1: Feature Flag (Simple)

This is the easiest way to enable AOT in a new project. It works by enabling the `aot` feature flag.

### Method 2: Pre-compiled Chunks (Production)

This is the approach used in the brevis-vera-zk project. The RISC-V program is pre-compiled into multiple chunks for maximum performance.

---

## Method 1: Feature Flag (Simple)

### Step 1: Add Dependencies

Add the following to your project's `Cargo.toml`:

```toml
[dependencies]
pico-sdk = { git = "https://github.com/brevis-network/pico" }
pico-vm = { git = "https://github.com/brevis-network/pico" }
pico-aot-dispatch = { git = "https://github.com/brevis-network/pico" }
pico-aot-runtime = { git = "https://github.com/brevis-network/pico" }

[features]
default = ["aot"]
aot = ["pico-aot-dispatch", "pico-aot-runtime/aot"]

# Optional optimizations
bigint-rug = ["pico-vm/bigint-rug"]
jemalloc = ["pico-vm/jemalloc"]
mmap-memory = ["pico-vm/mmap-memory"]
```

### Step 2: Use AotEmulatorCore in Code

```rust
use pico_vm::{Emulator, EmulatorConfig};
use pico_aot_runtime::AotEmulatorCore;

// 1. Load the compiled RISC-V program (ELF)
let program = std::fs::read("your_app.elf")?;

// 2. Create input data
let input_data = vec![/* your input bytes */];

// 3. Create AOT Emulator
let mut emu = AotEmulatorCore::new(program, input_data);

// 4. Run with AOT dispatch
pico_aot_dispatch::run_aot(&mut emu)?;

// 5. Get results from public values
let result = emu.public_values_stream;
```

---

## Method 2: Pre-compiled Chunks (Production)

This is the approach used in the brevis-vera-zk project for maximum performance. It involves pre-compiling the RISC-V program into multiple chunks.

### Architecture Overview

```
brevis-vera-guest (Rust RISC-V program)
    ↓ Compile to ELF
    cargo pico build
brevis-vera/zk-guest/app/elf/riscv32im-pico-zkvm-elf
    ↓ AOT Compile
    aot-chunks/ (34 pre-compiled chunks)
```

### Step 1: Build AOT Codegen Tool

Clone and build the pico repository:

```bash
# Clone pico
git clone https://github.com/brevis-network/pico
cd pico

# Build AOT codegen tool
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
- `aot-chunks/chunks/chunk_000` to `chunk_033` - 34 chunk crates

### Step 3: Configure Chunk Crates

Update each chunk's `Cargo.toml` to use git dependencies instead of local paths:

```bash
cd zk-guest/app/aot-chunks/chunks
for i in chunk_*; do
    sed -i '' 's|pico-aot-runtime = { path = "../../../aot-runtime" }|pico-aot-runtime = { git = "https://github.com/brevis-network/pico" }|' $i/Cargo.toml
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
# ... all other chunks
```

### Step 5: Add to Workspace

Add to your `Cargo.toml`:

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

This section documents how brevis-vra implements AOT.

### Current Status

- ✅ AOT chunks generated (34 chunks)
- ✅ Integrated into workspace
- ⏳ Prover code uses JIT (DefaultProverClient)

### Project Structure

```
brevis-vera/
├── Cargo.toml                      # Workspace root
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

### Build Commands

```bash
# Build with AOT chunks
cargo build --features pico

# Run tests
cargo test --features pico test_pico_crop --release

# Benchmark
cargo test --features pico --release -- --nocapture | grep "generated in"
```

---

## Comparison

| Feature | Feature Flag | Pre-compiled Chunks |
|---------|--------------|---------------------|
| **Ease of Use** | Simple | Complex |
| **Performance** | Good (faster than JIT) | Best (fully static optimization) |
| **Flexibility** | High | Low |
| **Best For** | Small/medium programs | Large programs, sharding |
| **Example Projects** | pico examples | brevis-vera-zk |

---

## Migration Steps

### From JIT to AOT (Feature Flag)

1. Add `pico-aot-dispatch` and `pico-aot-runtime` dependencies
2. Enable the `aot` feature in your `Cargo.toml`
3. Replace `Emulator` with `AotEmulatorCore`
4. Test that your program works correctly

### From Feature Flag to Pre-compiled Chunks

1. Run AOT compilation to generate chunks
2. Create chunk crates for each chunk
3. Create an AOT crate that depends on all chunks
4. Implement the `run_aot` function
5. Update your prover to use the new AOT module
6. Benchmark to ensure performance improvement

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

### Compilation Errors with Chunks

If chunk compilation fails, ensure all chunks use the same `pico-aot-runtime` source:

```bash
# Update all chunks
cd aot-chunks/chunks
for i in chunk_*; do
    sed -i '' 's|pico-aot-runtime = { path = ".*" }|pico-aot-runtime = { git = "https://github.com/brevis-network/pico" }|' $i/Cargo.toml
done
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
