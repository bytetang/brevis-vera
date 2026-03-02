# Design: ZKVM C2PA ECDSA P-256 Verification

## Context

### Current State
The current implementation performs C2PA signature verification on the host side:
1. Host loads image with C2PA metadata
2. `c2pa` library verifies ECDSA signature externally
3. Verification result (valid/invalid) is passed to ZKVM
4. ZKVM only performs **structural validation** (manifest format, algorithm check)

### Problem
This creates a trust assumption: verifiers must trust that the host performed genuine signature verification. A malicious host could:
- Return `true` without actually verifying
- Skip verification entirely
- Forge verification results

### Constraints
- Must work within Pico ZKVM constraints (RISC-V, limited resources)
- Must support ES256 (ECDSA P-256) algorithm as used by C2PA
- Test image: `test_img_small.JPG` with known public key

### Stakeholders
- ZK proof verifiers (need cryptographic guarantees)
- Image authenticity users (need trusted provenance)

## Goals / Non-Goals

**Goals:**
1. Move ECDSA P-256 signature verification inside ZKVM
2. Verify that the image hash was signed by the known public key
3. Output verification result as part of ZK proof public values
4. Maintain compatibility with existing C2PA structural validation

**Non-Goals:**
- Certificate chain validation (deferred to future work)
- Multiple signature algorithms (only ES256 for now)
- RSA-PSS or Ed25519 support (out of scope)
- Public input extraction (separate issue, as noted)

## Decisions

### D1: Use host-provided signature data vs. extract from C2PA

**Decision:** Extract signature data from C2PA manifest on host, pass to ZKVM

**Rationale:**
- C2PA manifest already contains the signature (r, s values)
- Host can parse the manifest store to extract signature components
- ZKVM receives: public key, signature (r, s), signed message hash

**Alternative Considered:** Have ZKVM parse C2PA manifest directly
- Rejected: Would require C2PA parsing logic in ZK guest, increases complexity

### D2: ECDSA verification approach

**Decision:** Use Pico ZKVM's built-in ECDSA verification via precompiles or brevis-network/signatures circuit

**Rationale:**
- Pico SDK provides secp256r1 (P-256) precompiles
- brevis-network/signatures provides optimized ECDSA circuit
- Reduces guest binary size

**Alternative Considered:** Implement ECDSA from scratch in guest
- Rejected: Complex, error-prone, larger circuit

### D3: Public key handling

**Decision:** Pass public key as hex string from host to ZKVM

**Rationale:**
- Public key is not sensitive (known to verifiers)
- Can be included in circuit for comparison
- Test case provides uncompressed public key (65 bytes, 04 prefix)

**Format:**
```
Public Key: 0460783afb3dba96bd37568481744eb0d8c0257261b8bc16dc96a6f50a867ea4bba3c6d8da159c60e5935399a394764baa6298eed36427269fb5a23c032d8815e9
- Prefix: 04 (uncompressed)
- X: 60783afb3dba96bd37568481744eb0d8c0257261b8bc16dc96a6f50a867ea4bba3 (32 bytes)
- Y: c6d8da159c60e5935399a394764baa6298eed36427269fb5a23c032d8815e9 (32 bytes)
```

### D4: Message to verify

**Decision:** Verify signature over the image SHA-256 hash

**Rationale:**
- C2PA signs the "claim" which includes the image hash
- The image hash is already computed for editing verification
- Simplifies circuit: just verify sig(image_hash)

## Risks / Trade-offs

### Risk 1: Circuit complexity
- ECDSA verification circuits are complex
- **Mitigation:** Use brevis-network/signatures which is well-tested

### Risk 2: Proof size/time increase
- Adding ECDSA verification increases proof generation time
- **Mitigation:** Accept trade-off for security; can optimize later

### Risk 3: Public key not matching
- Test image's public key might not match expected
- **Mitigation:** First verify with provided key; adjust if needed

### Risk 4: ZKVM resource constraints
- Pico ZKVM has limited memory for large circuits
- **Mitigation:** Use precompiles where possible; minimize circuit gates

## Migration Plan

### Phase 1: Host-side signature extraction
1. Add code to extract signature (r, s) from C2PA manifest
2. Pass signature data to prover input

### Phase 2: ZK guest verification
1. Add ECDSA verification circuit to guest program
2. Update `CircuitInput` to include signature data
3. Implement verification logic

### Phase 3: Integration and testing
1. Build ZK guest with new circuit
2. Test with `test_img_small.JPG`
3. Verify proof output includes cryptographic verification result

### Rollback
- If ECDSA circuit fails, can fall back to structural-only verification
- Keep existing `verify_c2pa` function as fallback

## Open Questions

1. **Q: How to extract signature (r, s) from C2PA manifest?**
   - Need to check c2pa library API for signature extraction
   - May need to parse manifest store JSON

2. **Q: What exactly is signed in C2PA?**
   - Is it the image hash directly?
   - Or a CBOR-encoded claim structure?
   - Need to verify the exact signing payload

3. **Q: Should the public key be hardcoded or configurable?**
   - For test: hardcoded is fine
   - For production: should be configurable per deployment

4. **Q: How to handle verification failure in the circuit?**
   - Should it halt or return false?
   - Current design: return false, allow proof to continue
