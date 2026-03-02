## 1. Project Setup
- [ ] 1.1 Initialize React + TypeScript project with Vite
- [ ] 1.2 Install dependencies: antd, axios, react-router-dom
- [ ] 1.3 Configure Ant Design theme
- [ ] 1.4 Set up project structure (components, pages, services, types)

## 2. API Service Layer
- [ ] 2.1 Create API client service (Axios)
- [ ] 2.2 Define TypeScript interfaces for API requests/responses
- [ ] 2.3 Implement Capture Layer API: `POST /api/v1/provenance/extract`
- [ ] 2.4 Implement Editing Layer APIs:
  - `POST /api/v1/edit/crop`
  - `POST /api/v1/edit/resize`
  - `POST /api/v1/edit/rotate`
- [ ] 2.5 Implement ZK Proof API: `POST /api/v1/zk/prove`

## 3. Image Upload Component
- [ ] 3.1 Create image upload component with drag-and-drop
- [ ] 3.2 Implement image preview
- [ ] 3.3 Add file type validation (JPEG, PNG)
- [ ] 3.4 Call Capture Layer API for C2PA metadata extraction

## 4. C2PA Provenance Display
- [ ] 4.1 Create provenance info card component
- [ ] 4.2 Display device information (camera model, manufacturer)
- [ ] 4.3 Display capture timestamp
- [ ] 4.4 Display signature algorithm (ES256, etc.)
- [ ] 4.5 Display verification status (verified/unverified)

## 5. Image Editor Component
- [ ] 5.1 Create editor layout with original/preview panels
- [ ] 5.2 Implement Crop tool
  - Interactive crop area selection
  - Coordinate input fields
- [ ] 5.3 Implement Resize tool
  - Width/height input fields
  - Maintain aspect ratio option
- [ ] 5.4 Implement Rotate tool
  - 90°/180°/270° buttons
- [ ] 5.5 Call Editing Layer APIs for image processing
  - Send crop/resize/rotate requests to backend
  - Receive edited image bytes
  - Build editing record for ZK proof

## 6. ZK Proof Generation
- [ ] 6.1 Create "Generate Proof" button
- [ ] 6.2 Call ZK Proof API: `POST /api/v1/zk/prove`
- [ ] 6.3 Add progress indicator (loading state)
- [ ] 6.4 Display generation time
- [ ] 6.5 Handle proof generation errors

## 7. Verification Result Display
- [ ] 7.1 Create result card component
- [ ] 7.2 Display c2pa_verified status (boolean)
- [ ] 7.3 Display editing_verified status (boolean)
- [ ] 7.4 Display number of operations applied
- [ ] 7.5 Display proof size (bytes)

## 8. Download Functionality
- [ ] 8.1 Add download edited image button
- [ ] 8.2 Add download ZK Proof button
- [ ] 8.3 Implement file blob handling

## 9. Integration Testing
- [ ] 9.1 Test full workflow: upload → edit → prove → verify
- [ ] 9.2 Test with sample image: imgs/test_img.JPG
- [ ] 9.3 Test error handling

## 10. Documentation
- [ ] 10.1 Add README with setup instructions
- [ ] 10.2 Document API endpoints used
- [ ] 10.3 Document component structure
