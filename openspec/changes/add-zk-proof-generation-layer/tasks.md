## 1. Implementation (ZK Circuit)
- [x] 1.1 Set up Pico ZKVM development environment
  - Reference: https://pico-docs.brevis.network/
  - Repo: https://github.com/brevis-network/pico
- [x] 1.2 Study Pico ZKVM proof flow
  - Learn: How to write ZK circuits in RISC Zero or similar framework
  - Learn: Circuit compilation and proving workflow
  - Learn: Input/output handling in ZKVMs
- [x] 1.3 Study brevis-network/signatures library
  - Reference: https://github.com/brevis-network/signatures
  - Learn: ECDSA P-256 circuit implementation
  - Learn: Circuit cost optimization for Pico
- [x] 1.4 Implement C2PA ECDSA verification circuit
  - Use brevis-network/signatures library
  - Optimize for circuit cost (Pico-compatible)
  - Input: signature, certificate chain, public key, image hash
  - Output: boolean (valid/invalid)
- [x] 1.5 Implement Crop proof circuit
  - Prove crop coordinates without revealing original dimensions
  - Input: original image hash, edited image hash, crop params
  - Output: boolean (crop verified)
- [x] 1.6 Implement Resize proof circuit
  - Prove resize dimensions without revealing scale factor
- [x] 1.7 Implement combined proof circuit
  - Combine C2PA verification + editing proof
  - Input: C2PA data + editing record + original/edited images
  - Output: ZK proof

## 2. Prover Implementation (Rust)
- [x] 2.1 Create Rust module under `src/zk/`
- [x] 2.2 Integrate Pico ZKVM prover SDK
  - Created `zk-guest/` standalone workspace (3-crate architecture)
  - `zk-guest/lib/` — shared types (`CircuitInput`, `PublicValuesStruct`)
  - `zk-guest/app/` — RISC-V guest program with pico-sdk
  - `PicoProver` in `src/zk/prover.rs` (feature-gated behind `pico`)
  - `SimulatedProver` remains default for development/testing
- [x] 2.3 Implement proof generation API
  - Input: original image, C2PA data, edited image, editing record
  - Output: ZK proof bytes
- [x] 2.4 Add proof serialization/deserialization

## 3. Testing
- [x] 3.1 Test C2PA verification circuit with sample image
  - Test image: `imgs/test_img.JPG` (Sony ILCE-1, has C2PA)
- [x] 3.2 Test editing proof circuits
- [x] 3.3 Test combined proof generation
- [x] 3.4 Benchmark proof generation time

## 4. Documentation
- [x] 4.1 Document circuit design
- [x] 4.2 Document prover API
- [x] 4.3 Add references to Pico and brevis-network/signatures
