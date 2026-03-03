# Extracting ECDSA Signature from C2PA Manifest

This document describes how to extract the raw ECDSA signature `(r, s)` from a C2PA-signed JPEG image by manually parsing the JUMBF/COSE binary structures.

## Background

The `c2pa` Rust library does **not** expose raw signature bytes through its public API. The internal `Claim::signature_val()` method is `pub(crate)`. To obtain the signature for ZKVM verification, we bypass the library and parse the binary structures directly from the file.

## Data Flow

```
JPEG file
  └─ APP11 markers (0xFFEB)              ← Step 1: Extract JUMBF segments
      └─ Reassembled JUMBF data           ← Step 2: Reassemble by (En, Z)
          └─ ISO BMFF box tree             ← Step 3: Parse boxes
              └─ c2pa.signature superbox   ← Step 4: Find by UUID "c2cs"
                  └─ CBOR content box      ← Step 5: Raw tagged CBOR
                      └─ COSE_Sign1        ← Step 6: Deserialize
                          └─ .signature    ← Step 7: r || s (P1363)
```

## Step 1: Extract JUMBF from JPEG APP11 Segments

C2PA embeds JUMBF data in JPEG APP11 markers (`0xFFEB`) per ITU-T T.84 Annex E.

Each APP11 segment has the following structure after the marker and length field (`Lp`):

| Field | Size | Description |
|-------|------|-------------|
| CI    | 2 bytes | Common identifier: `0x4A50` ("JP") |
| En    | 2 bytes | Box instance number (groups segments belonging to the same JUMBF box) |
| Z     | 4 bytes | Packet sequence number (ordering within a group) |
| Payload | variable | JUMBF box data fragment |

**Reassembly**: Group segments by `En`, sort by `Z` within each group, concatenate payloads.

```rust
fn extract_jumbf_from_jpeg(data: &[u8]) -> Option<Vec<u8>> {
    // 1. Verify SOI marker (0xFFD8)
    // 2. Scan markers until SOS (0xFFDA) or EOI (0xFFD9)
    // 3. For each APP11 (0xFFEB):
    //    - Check CI == 0x4A50
    //    - Read En (2 bytes), Z (4 bytes)
    //    - Collect payload
    // 4. Group by En, sort by Z, concatenate
}
```

## Step 2: Parse ISO BMFF Box Tree

The reassembled JUMBF data is a sequence of ISO BMFF boxes:

```
┌─────────┬──────────┬─────────────┐
│ LBox(4) │ TBox(4)  │ Payload ... │
└─────────┴──────────┴─────────────┘
```

- `LBox`: Box size in bytes (includes header). If `0` → extends to end; if `1` → 64-bit extended size follows.
- `TBox`: 4-byte ASCII box type (`jumb`, `jumd`, `cbor`, etc.)

JUMBF superboxes (`jumb`) contain nested children: a description box (`jumd`) followed by content boxes.

## Step 3: Locate the Signature Box

The `c2pa.signature` superbox is identified by its description box (`jumd`) containing the UUID:

```
c2cs UUID = 63 32 63 73 00 11 00 10 80 00 00 AA 00 38 9B 71
```

Or by the label string `"c2pa.signature"` in the description payload (fallback).

The JUMBF tree structure:

```
jumb (c2pa manifest store)
  └─ jumb (manifest, per claim)
      ├─ jumb (c2pa.claim, UUID "c2cl")
      │   └─ cbor → Claim CBOR bytes (the signed payload)
      ├─ jumb (c2pa.signature, UUID "c2cs")
      │   └─ cbor → COSE_Sign1 tagged CBOR
      └─ jumb (c2pa.assertions)
          └─ ...
```

## Step 4: Parse COSE_Sign1

The CBOR content box inside `c2pa.signature` contains a **COSE_Sign1** structure encoded as tagged CBOR (tag 18).

```
COSE_Sign1 = [
    protected:   bstr,   // CBOR-encoded header map {alg, x5chain}
    unprotected: map,    // {sigTst, rVals, pad}
    payload:     nil,    // Detached — the claim CBOR is the actual payload
    signature:   bstr    // The raw signature bytes
]
```

We use the `coset` crate to deserialize:

```rust
fn parse_ecdsa_from_cose_sign1(cose_cbor: &[u8]) -> Option<EcdsaSignature> {
    use coset::{CborSerializable, TaggedCborSerializable};

    let sign1 = coset::CoseSign1::from_tagged_slice(cose_cbor)
        .or_else(|_| coset::CoseSign1::from_slice(cose_cbor))
        .ok()?;

    let sig = &sign1.signature;

    match sig.len() {
        64 => Some(EcdsaSignature {        // ES256 (P-256)
            r: sig[..32].to_vec(),
            s: sig[32..].to_vec(),
        }),
        96 => Some(EcdsaSignature {        // ES384 (P-384)
            r: sig[..48].to_vec(),
            s: sig[48..].to_vec(),
        }),
        _ => None,
    }
}
```

## Step 5: Signature Format

For ECDSA, COSE stores signatures in **IEEE P1363 format** (raw `r || s`, no DER envelope):

| Algorithm | Signature Length | Format |
|-----------|----------------|--------|
| ES256 (P-256) | 64 bytes | r (32 bytes) ‖ s (32 bytes) |
| ES384 (P-384) | 96 bytes | r (48 bytes) ‖ s (48 bytes) |
| ES512 (P-521) | 132 bytes | r (66 bytes) ‖ s (66 bytes) |

This is different from the DER format used by OpenSSL. When verifying with OpenSSL, you must convert P1363 → DER.

## Verification

To verify the extracted signature, reconstruct the COSE **Sig_structure1** (the to-be-signed data):

```
Sig_structure1 = [
    "Signature1",           // context string
    bstr(protected),        // protected header bytes from COSE_Sign1
    bstr(external_aad),     // empty (b"")
    bstr(payload)           // the claim CBOR (detached payload)
]
```

Then verify: `ECDSA_Verify(public_key, SHA-256(CBOR(Sig_structure1)), (r, s))`

```rust
// Reconstruct Sig_structure1
let sig_structure = ciborium::Value::Array(vec![
    ciborium::Value::Text("Signature1".to_string()),
    ciborium::Value::Bytes(protected_bytes.to_vec()),
    ciborium::Value::Bytes(vec![]),      // external_aad
    ciborium::Value::Bytes(claim_cbor),  // detached payload
]);
let mut tbs_data = Vec::new();
ciborium::into_writer(&sig_structure, &mut tbs_data).unwrap();

// Verify with OpenSSL
let mut verifier = openssl::sign::Verifier::new(
    openssl::hash::MessageDigest::sha256(), &pkey
).unwrap();
verifier.update(&tbs_data).unwrap();
assert!(verifier.verify(&der_sig).unwrap());
```

The claim CBOR is the `c2pa.claim` box, found by UUID `c2cl` (`63 32 63 6C 00 11 00 10 80 00 00 AA 00 38 9B 71`).

## Implementation

The full implementation is in `src/provenance/parser.rs`:

| Function | Purpose |
|----------|---------|
| `extract_signature_from_file()` | Top-level: file → `EcdsaSignature` |
| `extract_jumbf_from_jpeg()` | JPEG APP11 → reassembled JUMBF bytes |
| `parse_boxes()` | JUMBF bytes → ISO BMFF box tree |
| `find_signature_cbor_in_jumbf()` | Box tree → COSE_Sign1 CBOR bytes |
| `find_claim_cbor_in_jumbf()` | Box tree → Claim CBOR bytes (detached payload) |
| `parse_ecdsa_from_cose_sign1()` | COSE_Sign1 CBOR → `EcdsaSignature { r, s }` |
| `extract_public_key_from_pem()` | PEM cert chain → uncompressed EC point (hex) |

## Dependencies

```toml
coset = "0.3"       # COSE_Sign1 deserialization
ciborium = "0.2"    # CBOR encoding (for Sig_structure1 reconstruction)
openssl = "0.10"    # X.509 cert parsing, ECDSA verification
hex = "0.4"         # Hex encoding
```

## Testing

```bash
# Run all parser tests including ECDSA verification
cargo test --release --lib provenance::parser -- --nocapture
```

The `test_verify_ecdsa_signature` test performs end-to-end verification:
1. Signs a test image with ES256
2. Extracts signature `(r, s)` and public key from the signed file
3. Reconstructs the COSE Sig_structure1
4. Verifies the ECDSA P-256 signature using OpenSSL

## See Also

- [docs/extra_public_key.md](extra_public_key.md) — Public key extraction from X.509 certificates
- [RFC 9052](https://www.rfc-editor.org/rfc/rfc9052) — COSE Structures and Process (COSE_Sign1)
- [ISO/IEC 19566-5](https://www.iso.org/standard/73604.html) — JUMBF (JPEG Universal Metadata Box Format)
- [C2PA Specification](https://c2pa.org/specifications/specifications/2.1/specs/C2PA_Specification.html)
