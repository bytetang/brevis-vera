# ECDSA P-256 Verification Guide

This guide covers how to verify ECDSA signatures using the NIST P-256 curve (also known as secp256r1 or prime256v1) using the ecdsa crate in this repository.

## Dependencies

Add the following to your `Cargo.toml`:

```toml
[dependencies]
ecdsa = { version = "0.16", features = ["std", "verifying"] }
p256 = "0.13"
signature = "2"
```

## Input Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| Public Key | `VerifyingKey<p256::NistP256>` | Loaded from SEC1-encoded bytes |
| Signature | `Signature<p256::NistP256>` | Loaded from bytes (supports ASN.1 DER) |
| Message | `&[u8]` | Raw message (automatically hashed with SHA-256) |

## Basic Usage

### 1. Load Public Key from Bytes (SEC1 format)

```rust
use ecdsa::{signature::Verifier, Signature, VerifyingKey};
use p256::NistP256;

// Public key bytes in SEC1 format (uncompressed: 0x04 || X || Y, or compressed: 0x02/0x03 || X)
let verifying_key = VerifyingKey::<NistP256>::from_sec1_bytes(&public_key_bytes)?;
```

Alternative methods:

```rust
// From encoded point
use p256::EncodedPoint;
let verifying_key = VerifyingKey::<NistP256>::from_encoded_point(&EncodedPoint::from_bytes(&pk_bytes)?);

// From affine coordinates
use p256::AffinePoint;
let verifying_key = VerifyingKey::<NistP256>::from_affine(AffinePoint::from_raw_coords(x, y)?)?;
```

### 2. Load Signature

```rust
// From DER-encoded bytes (most common for signatures from external systems)
let signature = Signature::<NistP256>::from_der(&signature_bytes)?;

// From raw bytes (R || S concatenation, 64 bytes for P-256)
let signature = Signature::<NistP256>::from_slice(&signature_bytes)?;
```

### 3. Verify Signature

```rust
use ecdsa::signature::Verifier;

// Verify message (automatically hashes with SHA-256)
verifying_key.verify(&message, &signature)?;
```

## Alternative Verification Methods

### Verify Pre-hashed Value

If you already have the hash of the message:

```rust
use ecdsa::signature::PrehashVerifier;

let msg_hash: [u8; 32] = /* your 32-byte hash */;
verifying_key.verify_prehash(&msg_hash, &signature)?;
```

### Verify with Custom Digest Algorithm

If you need to use a specific hash algorithm:

```rust
use ecdsa::signature::DigestVerifier;
use sha2::{Sha256, Digest};

// Create digest and feed message
let mut msg_digest = Sha256::new();
msg_digest.update(&message);
verifying_key.verify_digest(msg_digest, &signature)?;
```

Other supported digest algorithms: `Sha224`, `Sha256`, `Sha384`, `Sha512`.

## Complete Example

```rust
use ecdsa::{signature::Verifier, Signature, VerifyingKey};
use p256::NistP256;
use std::error::Error;

fn verify_signature(
    public_key_bytes: &[u8],
    signature_bytes: &[u8],
    message: &[u8],
) -> Result<bool, Box<dyn Error>> {
    // Load public key
    let vk = VerifyingKey::<NistP256>::from_sec1_bytes(public_key_bytes)?;

    // Load signature (assuming DER encoding)
    let sig = Signature::<NistP256>::from_der(signature_bytes)?;

    // Verify
    match vk.verify(message, &sig) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

fn main() {
    let pk = /* your public key bytes */;
    let sig = /* your signature bytes */;
    let msg = b"Hello, World!";

    match verify_signature(pk, sig, msg) {
        Ok(valid) => println!("Signature valid: {}", valid),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## Supported Formats

### Public Key Formats

- **SEC1 Uncompressed**: `0x04 || X (32 bytes) || Y (32 bytes)` = 65 bytes
- **SEC1 Compressed**: `0x02 || X (32 bytes)` or `0x03 || X (32 bytes)` = 33 bytes
- **ETH uncompleted**: Used by Ethereum (same as SEC1 uncompressed)

### Signature Formats

- **ASN.1 DER**: Standard encoding used by most systems
- **Raw (R || S)**: Concatenation of r and s values, each 32 bytes for P-256 = 64 bytes total

## Error Handling

The verification returns `Result<(), signature::Error>`. Common errors include:

- `Error::new()` - Generic signature error
- Invalid public key format
- Invalid signature format
- Signature verification failure (invalid signature)

## Related Modules

| File | Description |
|------|-------------|
| `ecdsa/src/verifying.rs` | VerifyingKey and verification interfaces |
| `ecdsa/src/lib.rs` | Core Signature type definitions |
| `ecdsa/src/hazmat.rs` | Low-level verification primitives |
| `ecdsa/src/recovery.rs` | Public key recovery functionality |
| `ecdsa/README.md` | Usage documentation |

## References

- NIST P-256: https://csrc.nist.gov/publications/detail/fips/186/4/final
- SECG SEC 1: https://www.secg.org/sec1-v2.pdf
- ECDSA OIDs:
  - ecdsa-sha224: 1.2.840.10045.4.3.1
  - ecdsa-sha256: 1.2.840.10045.4.3.2
  - ecdsa-sha384: 1.2.840.10045.4.3.3
  - ecdsa-sha512: 1.2.840.10045.4.3.4
