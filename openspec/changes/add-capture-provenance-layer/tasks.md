## 1. Implementation
- [ ] 1.1 Create Rust module structure under `src/provenance/`
- [ ] 1.2 Implement C2PA metadata parser
- [ ] 1.3 Implement media file reader (support JPEG, PNG)
- [ ] 1.4 Extract C2PA signature, certificate chain, and claims (C2PA uses ECDSA P-256)
- [ ] 1.5 Implement original image extraction
- [ ] 1.6 Create data struct for ZKVM input (signature + cert + image hash)

**Note**: ECDSA verification happens in ZKVM circuit (see ZK Proof Layer tasks)

## 2. Testing
- [ ] 2.1 Write unit tests for C2PA metadata parsing
- [ ] 2.2 Write integration tests with sample signed media
  - Test image: `imgs/test_img ILCE-1.JPG` (Sony, has C2PA metadata)
- [ ] 2.3 Test edge cases (missing metadata, malformed data)

## 3. Documentation
- [ ] 3.1 Add module documentation
- [ ] 3.2 Document output format for ZKVM
