## 1. Implementation
- [x] 1.1 Create Rust module structure under `src/provenance/`
- [x] 1.2 Implement C2PA metadata parser
- [x] 1.3 Implement media file reader (support JPEG, PNG)
- [x] 1.4 Extract C2PA signature, certificate chain, and claims (C2PA uses ECDSA P-256)
- [x] 1.5 Implement original image extraction
- [x] 1.6 Create data struct for ZKVM input (signature + cert + image hash)

**Note**: ECDSA verification happens in ZKVM circuit (see ZK Proof Layer tasks)

## 2. Testing
- [x] 2.1 Write unit tests for C2PA metadata parsing
- [x] 2.2 Write integration tests with sample signed media
  - Test image: `imgs/test_img.JPG` (Sony ILCE-1, has C2PA metadata with Es256)
- [x] 2.3 Test edge cases (missing metadata, malformed data)

## 3. Documentation
- [x] 3.1 Add module documentation
- [x] 3.2 Document output format for ZKVM
