# Pico zkVM Workflow Guide

## Overview

Pico is an open-source zero-knowledge virtual machine (zkVM) developed by Brevis Network, implementing the Glue-and-Coprocessor architecture. It uses Plonky3 as its proving backend and supports multiple proving fields.

---

## 1. Dependencies and Installation

### Requirements

- **Git**: Version control
- **Rust (Nightly)**: Required for compilation
- **Docker**: Optional, only needed for EVM proof generation

### Key Dependencies

**Core Dependencies (from workspace Cargo.toml):**

```toml
# Plonky3 (from brevis-network fork)
p3-air = { git = "https://github.com/brevis-network/Plonky3.git", rev = "f5056c9" }
p3-field = { git = "https://github.com/brevis-network/Plonky3.git", rev = "f5056c9" }
p3-baby-bear = { git = "...", features = ["nightly-features"], rev = "f5056c9" }
p3-koala-bear = { git = "...", features = ["nightly-features"], rev = "f5056c9" }
p3-mersenne-31 = { git = "...", rev = "f5056c9" }
p3-poseidon2 = { git = "...", rev = "f5056c9" }

# Other important dependencies
alloy-sol-types = "0.8.0"        # For Solidity types and ABI encoding
serde = { version = "1.0", features = ["derive"] }
bincode = "1.3.3"                # For serialization
```

### Project Dependencies Structure

A typical pico project has three crates:

**1. Library crate (lib/)** - Contains business logic

```toml
[dependencies]
alloy-sol-types = "0.8.0"
```

**2. App crate (app/)** - The zkVM application

```toml
[dependencies]
pico-sdk = { path = "../../../sdk/sdk", default-features = false, features = ["bb"] }
alloy-sol-types = "0.8.0"
fibonacci-lib = { path = "../lib" }
```

**3. Prover crate (prover/)** - Generates proofs

```toml
[dependencies]
pico-sdk = { path = "../../../sdk/sdk" }
bincode = "1.3.3"
serde = { version = "1.0", features = ["derive"] }
alloy-sol-types = "0.8.0"
fibonacci-lib = { path = "../lib" }
```

---

## 2. Writing Circuit Code

### Project Structure

```
my_project/
├── lib/           # Business logic (shared between host and zkVM)
│   ├── Cargo.toml
│   └── src/lib.rs
├── app/           # zkVM application
│   ├── Cargo.toml
│   └── src/main.rs
└── prover/        # Proof generation
    ├── Cargo.toml
    └── src/main.rs
```

### Step 1: Define Public Values (lib/src/lib.rs)

```rust
use alloy_sol_types::sol;

// Define the public values struct that will be committed to the proof
sol! {
    struct PublicValuesStruct {
        uint32 n;
        uint32 a;
        uint32 b;
    }
}

// Your business logic function
pub fn fibonacci(mut a: u32, mut b: u32, n: u32) -> (u32, u32) {
    for _ in 0..n {
        let next = a.wrapping_add(b);
        a = b;
        b = next;
    }
    (a, b)
}

// Helper function to load ELF
pub fn load_elf(path: &str) -> Vec<u8> {
    std::fs::read(path).unwrap_or_else(|err| {
        panic!("Failed to load ELF file from {}: {}", path, err);
    })
}
```

### Step 2: Write the zkVM Application (app/src/main.rs)

```rust
#![no_main]  // Required: prevent standard Rust main()

use alloy_sol_types::SolValue;
use fibonacci_lib::{fibonacci, PublicValuesStruct};
use pico_sdk::io::{commit_bytes, read_as};

pico_sdk::entrypoint!(main);

pub fn main() {
    // Read input from the environment
    let n: u32 = read_as();

    // Execute computation
    let (a_result, b_result) = fibonacci(0, 1, n);

    // Encode and commit results to public values
    let result = PublicValuesStruct {
        n,
        a: a_result,
        b: b_result,
    };
    let encoded_bytes = result.abi_encode();
    commit_bytes(&encoded_bytes);
}
```

### Key I/O Functions

| Function | Description |
|----------|-------------|
| `read_as::<T>()` | Read and deserialize input |
| `read_vec()` | Read raw bytes from input |
| `commit_bytes(&buf)` | Commit bytes to public values |
| `commit(&value)` | Commit serializable struct |
| `pico_sdk::entrypoint!(main)` | Define zkVM entry point |

### Available Cryptographic Precompiles

| Precompile | Code | Description |
|------------|------|-------------|
| SHA_EXTEND | 0x00300105 | SHA-256 extend |
| SHA_COMPRESS | 0x00010106 | SHA-256 compress |
| KECCAK_PERMUTE | 0x00010109 | Keccak hash |
| ED_ADD | 0x00010107 | Ed25519 point addition |
| ED_DECOMPRESS | 0x00000108 | Ed25519 decompression |
| SECP256K1_ADD | 0x0001010A | secp256k1 point addition |
| SECP256K1_DOUBLE | 0x0000010B | secp256k1 point doubling |
| SECP256K1_DECOMPRESS | 0x0000010C | secp256k1 decompression |
| SECP256R1_ADD | 0x00010130 | NIST P-256 addition |
| SECP256R1_DOUBLE | 0x00000131 | NIST P-256 doubling |
| BN254_ADD | 0x0001010E | BN254 point addition |
| BN254_DOUBLE | 0x0000010F | BN254 point doubling |
| BLS12381_ADD | 0x0001011E | BLS12-381 addition |
| BLS12381_DOUBLE | 0x0000011F | BLS12-381 doubling |
| POSEIDON2_PERMUTE | 0x0001012F | Poseidon2 hash |
| UINT256_MUL | 0x0001011D | 256-bit multiplication |
| COMMIT | 0x00000010 | Commit to public values |
| VERIFY_PICO_PROOF | 0x0000001B | Verify nested proofs |
| ENTER_UNCONSTRAINED | 0x00000003 | Enter unconstrained mode |
| EXIT_UNCONSTRAINED | 0x00000004 | Exit unconstrained mode |

---

## 3. Proof Generation Flow

### Prover Chain Architecture

The proof generation follows a multi-phase pipeline:

```
+-------------+    +-------------+    +-------------+    +-------------+    +-------------+
|   RISCV     |-->|   CONVERT   |-->|   COMBINE    |-->|   COMPRESS  |-->|    EMBED    |
|   (EXEC)    |   | (RECURSION) |   | (RECURSION) |   | (RECURSION) |   |  (EVM/G16)  |
+-------------+    +-------------+    +-------------+    +-------------+    +-------------+
```

1. **RISCV Phase**: Executes RISC-V program, generates initial STARK proof
2. **CONVERT (Recursion)**: Converts recursive STARK to aggregate proofs
3. **COMBINE (Recursion)**: Combines multiple proofs
4. **COMPRESS (Recursion)**: Compresses to smaller proof
5. **EMBED (Onchain)**: Converts to Groth16 for EVM verification

### Prover Code (prover/src/main.rs)

```rust
use alloy_sol_types::SolType;
use fibonacci_lib::{fibonacci, load_elf, PublicValuesStruct};
use pico_sdk::{client::DefaultProverClient, init_logger};

fn main() {
    // Initialize logger
    init_logger();

    // Load ELF binary
    let elf = load_elf("../app/elf/riscv32im-pico-zkvm-elf");

    // Create prover client
    let client = DefaultProverClient::new(&elf);
    let stdin_builder = client.get_stdin_builder();

    // Write input
    let n = 100u32;
    stdin_builder.borrow_mut().write(&n);

    // Generate fast proof (RISCV phase only)
    let proof = client.prove_fast().expect("Failed to generate proof");

    // Decode public values from proof
    let public_buffer = proof.pv_stream.unwrap();
    let public_values = PublicValuesStruct::abi_decode(&public_buffer, true).unwrap();

    // Verify
    println!("n: {}, a: {}, b: {}", public_values.n, public_values.a, public_values.b);
}
```

### Prover Client API

```rust
// Create client
let client = DefaultProverClient::new(&elf);

// Fast proof (RISCV phase only - recommended for development)
let proof = client.prove_fast(stdin)?;

// Full proof chain (RISCV + all recursion phases)
let (riscv_proof, embed_proof) = client.prove(stdin)?;

// Verify proof
client.verify(&(riscv_proof, embed_proof))?;

// Generate EVM-compatible Groth16 proof (requires Docker)
client.prove_evm(stdin, need_setup, output_path, field_type)?;
```

### Supported Fields

| Field | Feature | Prime | Status |
|-------|---------|-------|--------|
| KoalaBear | `kb` | p = 2^31 - 2^24 + 1 | Full support |
| BabyBear | `bb` | p = 2^31 - 2^27 + 1 | Full support |
| Mersenne31 | `m31` | p = 2^31 - 1 | RISCV only |

---

## 4. CLI Usage

### Installation

```bash
# Install pico CLI (adds cargo pico command)
cargo install --path sdk/cli
```

### Available Commands

#### Build ELF Binary

```bash
# Basic build
cargo pico build

# With features
cargo pico build --features bb

# With custom ELF name
cargo pico build --elf-name my_program

# Output to custom directory
cargo pico build --output-directory ./output
```

**Build Options:**

- `--features`: Features to activate (bb, kb, m31)
- `--elf-name`: Custom ELF binary name
- `--output-directory`: Output directory (default: `elf`)
- `--binary`: Build specific binary
- `--rustflags`: Extra rustc flags

#### Generate Proof

```bash
# Fast proof (RISCV phase only)
cargo pico prove --elf <path> --input <file/hex> --fast

# Full proof chain
cargo pico prove --elf <path> --input <file>

# EVM proof (requires Docker)
cargo pico prove --elf <path> --input <file> --evm

# With Groth16 setup
cargo pico prove --elf <path> --input <file> --evm --setup

# Specify field
cargo pico prove --elf <path> --input <file> --field kb
```

**Prove Options:**

- `--elf`: ELF file path
- `--input`: Input bytes or file path (supports hex with `0x` prefix)
- `--fast`: Fast proving mode (RISCV phase only)
- `--evm`: Generate EVM-compatible Groth16 proof
- `--setup`: Run Groth16 circuit setup (must use with `--evm`)
- `--field`: Field type (bb, kb, m31, default: kb)
- `--output`: Proof output directory

#### Other Commands

```bash
# Create new project
cargo pico new

# Run emulator test
cargo pico test-emulator
```

---

## 5. Complete Example: Fibonacci

### lib/Cargo.toml

```toml
[package]
name = "fibonacci-lib"
version = "1.0.0"
edition = "2021"

[dependencies]
alloy-sol-types = "0.8.0"
```

### lib/src/lib.rs

```rust
use alloy_sol_types::sol;

sol! {
    struct PublicValuesStruct {
        uint32 n;
        uint32 a;
        uint32 b;
    }
}

pub fn fibonacci(mut a: u32, mut b: u32, n: u32) -> (u32, u32) {
    for _ in 0..n {
        let next = a.wrapping_add(b);
        a = b;
        b = next;
    }
    (a, b)
}

pub fn load_elf(path: &str) -> Vec<u8> {
    std::fs::read(path).unwrap()
}
```

### app/Cargo.toml

```toml
[package]
name = "fibonacci-app"
version = "1.0.0"
edition = "2021"

[dependencies]
pico-sdk = { path = "../../../sdk/sdk", default-features = false, features = ["bb"] }
alloy-sol-types = "0.8.0"
fibonacci-lib = { path = "../lib" }
```

### app/src/main.rs

```rust
#![no_main]

use alloy_sol_types::SolValue;
use fibonacci_lib::{fibonacci, PublicValuesStruct};
use pico_sdk::io::{commit_bytes, read_as};

pico_sdk::entrypoint!(main);

pub fn main() {
    let n: u32 = read_as();
    let (a_result, b_result) = fibonacci(0, 1, n);

    let result = PublicValuesStruct {
        n,
        a: a_result,
        b: b_result,
    };
    commit_bytes(&result.abi_encode());
}
```

### prover/Cargo.toml

```toml
[package]
name = "fibonacci-prover"
version = "1.0.0"
edition = "2021"

[dependencies]
pico-sdk = { path = "../../../sdk/sdk" }
fibonacci-lib = { path = "../lib" }
serde = { version = "1.0", features = ["derive"] }
alloy-sol-types = "0.8.0"
```

### prover/src/main.rs

```rust
use alloy_sol_types::SolType;
use fibonacci_lib::{fibonacci, load_elf, PublicValuesStruct};
use pico_sdk::{client::DefaultProverClient, init_logger};

fn main() {
    init_logger();
    let elf = load_elf("../app/elf/riscv32im-pico-zkvm-elf");

    let client = DefaultProverClient::new(&elf);
    let stdin_builder = client.get_stdin_builder();
    stdin_builder.borrow_mut().write(&100u32);

    let proof = client.prove_fast().expect("Failed to generate proof");

    let public_buffer = proof.pv_stream.unwrap();
    let public_values = PublicValuesStruct::abi_decode(&public_buffer, true).unwrap();

    println!("Result: n={}, a={}, b={}", public_values.n, public_values.a, public_values.b);
}
```

### Build and Run

```bash
# Build the ELF
cd app && cargo pico build

# Generate proof
cd ../prover && cargo run
```

---

## 6. Key Patterns for AI Coding

### Field Selection

```toml
# Choose ONE field
pico-sdk = { features = ["bb"] }  # BabyBear
pico-sdk = { features = ["kb"] }  # KoalaBear (default)
pico-sdk = { features = ["m31"] } # Mersenne31
```

### Input/Output Patterns

```rust
// Read input
let n: u32 = read_as();           // Bincode deserialized
let data: Vec<u8> = read_vec();   // Raw bytes

// Commit output
commit_bytes(&encoded);            // Raw bytes
commit(&struct_instance);          // Serializable struct
```

### Error Handling

```rust
// Deserialization can panic, handle with expect/unwrap
let value: MyType = read_as();    // Panics on failure

// Or use read_vec for manual handling
let data = read_vec();
let value = bincode::deserialize(&data).ok();
```

### Multiple Inputs

```rust
// Read multiple inputs sequentially
let a: u32 = read_as();
let b: u32 = read_as();
let data: Vec<u8> = read_vec();
```

### Cycle Tracking (for debugging)

```rust
println!("cycle-tracker-start: computation_name");
let result = compute_something();
println!("cycle-tracker-end: computation_name");
```

---

This guide provides all the essential patterns and workflows needed for AI-assisted pico development.
