# Project Context

## 1. System Overview

### Purpose

**Brevis Vera** is an end-to-end digital media authenticity attestation system designed to solve a critical problem in the modern digital age: **how can we verify that a piece of digital media is authentic, even after it has been edited?**

In an era where AI-generated and manipulated media are increasingly prevalent, there is a growing need for:
- **Content authenticity**: Proving that media originated from a real device capture
- **Editing transparency**: Demonstrating that certain edits were applied without revealing sensitive details
- **Privacy-preserving verification**: Enabling third parties to verify authenticity without trusting the editor

Brevis Vera bridges the gap between **hardware-level provenance** (C2PA-style signatures) and **software-level editing verification** using **Zero-Knowledge Proofs**.

---

### Why ZKVM for Attestation?

#### Traditional Approaches & Their Limitations

| Approach | How It Works | Limitations |
|----------|--------------|-------------|
| **Watermarking** | Embed invisible markers in images | Can be removed by cropping or recompression |
| **Metadata Logging** | Record edit history in metadata | Metadata can be easily faked or removed |
| **Server-Side Verification** | Keep original on server, compare | Requires trust in server, privacy concerns |
| **Digital Signatures Only** | Sign the original + edited version | Reveals too much information about edits |

#### Benefits of ZKVM-Based Attestation

| Benefit | Description |
|---------|-------------|
| **Privacy-Preserving** | Prove that edits were applied without revealing the original image or specific edit parameters |
| **Tamper-Proof** | The proof itself is cryptographically secure and cannot be forged |
| **Non-Interactive Verification** | Anyone can verify the proof without needing to interact with the original editor |
| **Minimal Trust** | Verifier only needs to trust the ZKVM circuit logic, not the editor |
| **Efficient** | Proof size is small and verification is fast |
| **Selective Disclosure** | Choose what to reveal: e.g., "a crop was applied" without showing original dimensions |

#### Comparison: Traditional vs ZKVM

| Aspect | Traditional Metadata | ZKVM Proof |
|--------|---------------------|------------|
| **Privacy** | Metadata can be inspected | Proof reveals nothing beyond validity |
| **Tamper Resistance** | Metadata can be modified | Cryptographically binding |
| **Trust Model** | Requires trusting the editor | Trustless verification |
| **Verification** | Can be bypassed | Cannot be faked |
| **Complexity** | Simple but weak | More complex but robust |

---

## 2. Glossary of Technical Terms

| Term | English | Description |
|------|---------|-------------|
| 数字真实性证明 | Provenance / Provenance Attestation | Proves the source and authenticity of digital media, ensuring content comes from a real device rather than being fabricated |
| 零知识证明 | Zero-Knowledge Proof (ZK Proof) | A cryptographic proof that allows one party to prove a statement is true without revealing any information beyond the validity of the statement |
| 零知识虚拟机 | ZKVM (Zero-Knowledge Virtual Machine) | A virtual machine that executes zero-knowledge proofs, such as Pico ZKVM |
| C2PA | Coalition for Content Provenance and Authenticity | An alliance that sets standards for digital media provenance |
| 编辑证明 | Proof of Editing | A zero-knowledge proof that proves media has been edited without exposing specific editing details |
| 证书链 | Certificate Chain | A series of digital certificates needed to verify a signature, a trust chain from root certificate to end certificate |
| ECDSA | Elliptic Curve Digital Signature Algorithm | An elliptic curve digital signature algorithm used for generating and verifying signatures |
| Merkle 树 | Merkle Tree | A hash tree structure used for efficiently verifying data integrity in large datasets |
| 图像裁剪 | Crop | An image editing operation that removes certain parts of an image |
| 原像 | Original / Source Image | An unedited original image |
| 编辑后图像 | Edited Image | An image after editing processing |

---

## 3. System Architecture

```
┌────────────────────────────────────────────────────────────────────────────────┐
│                              Brevis Vera System                                 │
├────────────────────────────────────────────────────────────────────────────────┤
│                                                                                │
│  ┌──────────────────┐     ┌──────────────────┐     ┌──────────────────────┐  │
│  │   User/Device    │     │   Brevis Vera    │     │     Verifier        │  │
│  │   (Editor)       │     │     System       │     │   (Third Party)     │  │
│  └────────┬─────────┘     └────────┬─────────┘     └──────────┬─────────┘  │
│           │                         │                          │             │
│           │  1. Signed Media        │                          │             │
│           │ ──────────────────────▶ │                          │             │
│           │                         │                          │             │
│           │                         │  ┌────────────────────┐  │             │
│           │                         │  │ Capture &          │  │             │
│           │                         │  │ Provenance Layer   │  │             │
│           │                         │  │ - C2PA/ECDSA Verify│  │             │
│           │                         │  │ - Extract metadata │  │             │
│           │                         │  └────────────────────┘  │             │
│           │                         │            │              │             │
│           │  2. Edit Request        │            ▼              │             │
│           │ ──────────────────────▶│  ┌────────────────────┐  │             │
│           │                         │  │ Editing Layer      │  │             │
│           │                         │  │ - Crop/Resize/etc  │  │             │
│           │                         │  │ - Generate edited  │  │             │
│           │                         │  └────────────────────┘  │             │
│           │                         │            │              │             │
│           │                         │            ▼              │             │
│           │                         │  ┌────────────────────┐  │             │
│           │                         │  │ ZK Proof Layer     │  │             │
│           │                         │  │ - Pico ZKVM        │  │             │
│           │                         │  │ - Generate proof   │  │             │
│           │                         │  └────────────────────┘  │             │
│           │                         │            │              │             │
│           │  3. Edited + Proof     │            │              │             │
│           │ ◀──────────────────────│            │              │             │
│           │                         │            │              │             │
│           │                         │            ▼              │             │
│           │                         │  ┌────────────────────┐  │             │
│           │                         │  │ Storage/API        │  │             │
│           │                         │  │ - Store proofs     │  │             │
│           │                         │  │ - Serve to verifiers│ │             │
│           │                         │  └────────────────────┘  │             │
│           │                         │                          │             │
│           │                         │  4. Proof + Image       │             │
│           │                         │ ────────────────────────▶ │             │
│           │                         │                          │             │
│           │                         │                          │  5. Verify  │
│           │                         │                          │ ──────────▶ │
│           │                         │                          │             │
│           │                         │  6. Verdict: VALID/INVALID              │
│           │                         │ ◀───────────────────────│             │
│           │                         │                          │             │
└───────────┴─────────────────────────┴──────────────────────────┴─────────────┘
```

---

## 4. Technology Stack

### Programming Languages

| Language | Use Case |
|----------|----------|
| **Rust** | Image processing library (Photon), ZK proof generation, core business logic |
| **Go** | Backend services, API servers, CLI tools |
| **JavaScript/TypeScript** | Frontend UI, Web verification interface |
| **Solidity** | Blockchain contracts (for on-chain verification if needed) |

### Key Technologies

| Technology | Use Case |
|------------|----------|
| **Pico ZKVM** | Zero-knowledge proof generation engine |
| **Photon (Rust)** | Image processing library, supports crop, filters, and other transformations |
| **ECDSA (P-256)** | Digital signature verification (simulating C2PA signature verification) |
| **Go-Ethereum** | Blockchain interaction (for on-chain verification) |
| **gRPC** | Service-to-service communication |
| **React/Vue** | Web verification interface |
| **SQLite/PostgreSQL** | Data storage |

---

## 5. Function Module Implementation

### 5.1 Capture & Provenance Layer

```
Functionality:
├── Accept media files with provenance metadata
├── Parse C2PA/ECDSA signature data
├── Verify signature validity (ECDSA P-256)
└── Extract original image and metadata

Implementation:
├── Rust: Use ring or k256 library for ECDSA signature verification
├── Parse JSON/binary metadata
└── Output: Verification result + original image
```

### 5.2 Editing Layer

```
Functionality:
├── Load original image
├── Support at least 2 image transformations (Crop is mandatory)
├── Optional: rotation, scaling, filters, etc.
└── Output edited image

Implementation:
├── Rust: Use Photon library
├── Support Crop, Resize
└── Output: Edited image + editing type record
```

### 5.3 ZK Proof Generation Layer

```
Functionality:
├── Generate consistency proof between before and after editing
├── Prove specific types of edits were applied (e.g., crop)
├── Hide specific details of edits
└── Generate verifiable proof file

Implementation:
├── Rust: Integrate Pico ZKVM
├── Write ZK circuit/program
├── Input: Original image hash + edited image + edit type
└── Output: ZK Proof
```

### 5.4 Verification Layer

```
Functionality:
├── Accept edited image + ZK Proof
├── Verify proof validity
├── Verify signature source authenticity
└── Output final authenticity verdict

Implementation:
├── Go/TypeScript: CLI or Web interface
├── Integrate ZKVM verifier
├── Verify ECDSA signature
└── Output: Valid / Invalid + details
```

---

## 6. User Flow

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Upload   │ ──▶ │  Signature  │ ──▶ │   Image    │ ──▶ │    ZK      │
│   Image    │     │  Verify     │     │   Edit     │ ──▶ │   Proof    │
│ (w/metadata)│     │  (ECDSA)    │     │ (Crop/etc) │     │  Generation│
└─────────────┘     └─────────────┘     └─────────────┘     └─────────────┘
                                                                  │
                                                                  ▼
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Verify   │ ◀── │  Authent-  │ ◀── │    ZK      │ ◀── │   Save     │
│   Result   │     │ icity Verdict│    │  Verify    │     │   Proof    │
│(display to │     │Valid/Invalid│    │            │     │            │
│   user)    │     │            │     │            │     │            │
└─────────────┘     └─────────────┘     └─────────────┘     └─────────────┘
```

---

## 7. Suggested File Structure

```
brevis-vera/
├── src/
│   ├── provenance/       # Signature verification module
│   │   └── signer.rs     # ECDSA verification
│   ├── editor/           # Image editing module
│   │   └── processor.rs  # Photon integration
│   ├── zk/               # ZK proof module
│   │   ├── circuit/      # ZK circuit
│   │   └── prover.rs     # Proof generation
│   └── verifier/         # Verification module
│       └── verify.rs     # Proof verification
├── web/                  # Web verification UI
├── cli/                  # CLI tools
└── contracts/            # Blockchain contracts (if needed)
```

---

## 8. Reference Resources

- Photon Image Library: https://github.com/silvia-odwyer/photon/
- Pico ZKVM Documentation: https://pico-docs.brevis.network/
- C2PA Standard: https://c2pa.org/
- ECDSA P-256: NIST Standard Curve, Pico VM ecdsa patch: https://github.com/brevis-network/signatures
