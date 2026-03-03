# Extracting ECDSA Public Key from C2PA Manifest

This document describes how to extract the ECDSA P-256 public key from a C2PA-signed image.

## Overview

When verifying C2PA provenance inside the ZKVM, we need both:
- **Public Key**: Can be extracted from the X.509 certificate chain
- **Signature (r, s)**: Not directly exposed by c2pa library (see limitations)

## Implementation

The extraction is implemented in `src/provenance/parser.rs`:

```rust
/// Extract ECDSA signature (r, s) and public key from the C2PA manifest.
fn extract_ecdsa_data(manifest: &c2pa::Manifest) -> (Option<EcdsaSignature>, Option<String>)
```

### Step 1: Get Certificate Chain

```rust
let sig_info = manifest.signature_info()?;
let cert_chain = sig_info.cert_chain();  // Returns PEM-encoded certificate chain
```

### Step 2: Parse X.509 Certificate

```rust
use openssl::x509::X509;

// Find the first certificate in PEM chain
let cert_start = pem_cert_chain.find("-----BEGIN CERTIFICATE-----")?;
let cert_end = cert_start + pem_cert_chain[cert_start..].find("-----END CERTIFICATE-----")? + 25;
let pem_cert = &pem_cert_chain[cert_start..cert_end];

// Parse the certificate
let cert = X509::from_pem(pem_cert.as_bytes()).ok()?;
```

### Step 3: Extract Public Key

```rust
use openssl::ec::{EcKey, PointConversionForm};
use openssl::bn::BigNumContext;

// Get the public key from the certificate
let pub_key = cert.public_key().ok()?;

// Extract EC key components
let ec_key = pub_key.ec_key().ok()?;
let ec_group = ec_key.group();
let ec_point = ec_key.public_key();

// Convert to uncompressed form (65 bytes: 0x04 || X || Y)
let mut ctx = BigNumContext::new().ok()?;
let point_bytes = ec_point.to_bytes(
    ec_group,
    PointConversionForm::UNCOMPRESSED,
    &mut ctx
).ok()?;

// Verify it's uncompressed (starts with 0x04)
assert_eq!(point_bytes[0], 0x04);

// Convert to hex string
let public_key_hex = hex::encode(&point_bytes);
```

## Result

The extracted public key is:
- **Format**: Uncompressed ECDSA P-256 point
- **Length**: 65 bytes (130 hex characters)
- **Prefix**: `04` (indicates uncompressed form)

Example:
```
0460783afb3dba96bd37568481744eb0d8c0257261b8bc16dc96a6f50a867ea4bba3c6d8da159c60e5935399a394764baa6298eed36427269fb5a23c032d8815e9
```

## Limitations

### Signature (r, s) Not Available

The c2pa library does **not** expose the raw signature bytes through its public API. The signature is stored in:
- JUMBF (JPEG Universal Metadata Box Format) binary boxes
- COSE (CBOR Object Signing and Encryption) format inside `c2pa.signature` box

**Investigation Results**:
- The internal `Claim` struct has `signature_val()` method but it's `pub(crate)` - not accessible externally
- The signature is referenced via URL: `self#jumbf=/c2pa/urn:uuid:.../c2pa.signature`
- JUMBF boxes can be loaded via `c2pa::jumbf_io::load_jumbf_from_file()`
- The signature is stored in COSE_Sign1 format within nested JUMBF/CBOR structures
- Parsing requires manual JUMBF box traversal and COSE decoding

**Workarounds**:
1. **For testing**: Manually provide known signature data
2. **Future**: Implement full JUMBF/COSE binary parsing (requires significant effort)
3. **Alternative**: Use c2pa's internal verification result as a proxy

### Why c2pa Library Doesn't Expose Signatures

The library's design is for **verification**, not **extraction**:
- It verifies signatures internally using OpenSSL
- It doesn't expose raw cryptographic components by default
- This is a security best practice (reduces attack surface)

## Testing

Run the debug example to verify extraction:

```bash
cargo run --release --example debug_manifest
```

Expected output:
```
=== SIGNATURE INFO ===
Issuer: Some("Default Certificate")
Algorithm: Some("es256")
Public Key: Some("0460783afb3dba96bd37568481744eb0d8c0257261b8bc16dc96a6f50a867ea4bba3c6d8da159c60e5935399a394764baa6298eed36427269fb5a23c032d8815e9")

=== ZKVM INPUT ===
ECDSA Signature: None
Public Key: Some("0460783afb3dba96bd37568481744eb0d8c0257261b8bc16dc96a6f50a867ea4bba3c6d8da159c60e5935399a394764baa6298eed36427269fb5a23c032d8815e9")
```

## Dependencies

```toml
openssl = "0.10"
```

## See Also

- [C2PA Specification](https://c2pa.org/)
- [OpenSSL EC Key Documentation](https://docs.rs/openssl/0.10/)
- [ZKVM ECDSA Verification](zk-guest/app/src/main.rs) - ZKVM-side verification logic
