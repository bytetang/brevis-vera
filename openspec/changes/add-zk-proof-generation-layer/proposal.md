# Change: Add ZK Proof Generation Layer

## Why
The ZK Proof Generation Layer is the core of Brevis Vera's privacy-preserving attestation. It generates zero-knowledge proofs that verify both:
1. **C2PA Provenance**: The original image came from an authenticated device (verified inside ZKVM)
2. **Editing Proof**: The claimed edits were applied correctly without revealing sensitive details

This layer combines C2PA verification and editing verification into a single ZK proof that can be verified trustlessly.

## What Changes
- **NEW**: Add `zk-proof` capability for ZK proof generation
- Integrate Pico ZKVM for proof generation
- Implement C2PA verification circuit (using brevis-network/signatures)
- Implement editing proof circuit (prove crop/resize/rotate without revealing details)
- Generate combined proof: C2PA verification + editing proof
- Output ZK proof file for verification

## Impact
- Affected specs: New capability `zk-proof`
- Affected code: New Rust module under `src/zk/`
- Dependencies: Pico ZKVM, brevis-network/signatures (ECDSA P-256)

## Required Learning Resources

### Core Documentation
| Resource | URL | Purpose |
|----------|-----|---------|
| Pico ZKVM Docs | https://pico-docs.brevis.network/ | Main documentation |
| Pico GitHub | https://github.com/brevis-network/pico | Source code & examples |
| brevis-network/signatures | https://github.com/brevis-network/signatures | ECDSA P-256 circuits |

### Topics to Study
1. **ZK Circuit Development**
   - How ZKVMs work (ZK execution model)
   - Circuit programming paradigm
   - Constraint systems

2. **Pico ZKVM Specific**
   - Proof generation workflow
   - Input/output serialization
   - Circuit compilation

3. **ECDSA in ZK**
   - Signature verification in circuits
   - Circuit cost optimization
   - Security considerations

4. **Image Processing in ZK**
   - Image hashing (Poseidon, SHA)
   - Merkle tree proofs
   - Range proofs for dimensions

### Suggested Learning Path
```
Week 1: Pico ZKVM fundamentals
  → Read docs, run hello-world example

Week 2: brevis-network/signatures
  → Study ECDSA circuit implementation
  → Understand circuit cost optimization

Week 3: Custom circuits (Crop/Resize)
  → Design circuit logic
  → Implement and test

Week 4: Integration
  → Combine circuits
  → End-to-end testing
```
