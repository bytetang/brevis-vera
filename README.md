# brevis-vera

End-to-end digital media authenticity attestation system.

## Getting Started

### Backend Server

```bash
# Start the Rust backend server (runs on http://localhost:8080)
cargo run --release
```

### Frontend

```bash
# Navigate to frontend directory
cd frontend

# Install dependencies (if not already installed)
npm install

# Start the development server (runs on http://localhost:5173)
npm run dev
```

## API Endpoints

- `GET  /health` - Health check
- `POST /api/v1/edit/crop` - Crop image
- `POST /api/v1/edit/resize` - Resize image
- `POST /api/v1/edit/rotate` - Rotate image
- `POST /api/v1/provenance/extract` - Extract provenance data
- `POST /api/v1/zk/prove` - Generate ZK proof
