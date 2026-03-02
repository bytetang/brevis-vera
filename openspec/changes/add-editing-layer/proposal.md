# Change: Add Editing Layer

## Why
The Editing Layer is the second stage in the Brevis Vera pipeline. After verifying the provenance of the original media, users need to apply edits (crop, resize, filters, etc.) to create the final output. This layer transforms the original image while recording the editing operations for ZK proof generation.

## Architecture Decision: Backend API vs Frontend

### Option A: Backend API (Editing performed on server)

```
Frontend → [Edit Request API] → Backend (Photon) → Edited Image
```

| Pros | Cons |
|------|------|
| Single source of truth - backend controls exact operations | Network latency for each edit |
| Easier ZK proof generation - backend knows exact operations | Higher server load |
| Security - original image never leaves server | Slower UX feedback |
| Consistency - same results across all clients | Scalability concerns |

### Option B: Frontend (Editing performed in browser)

```
Frontend (Canvas/JS) → [Edit Record API] → Backend
```

| Pros | Cons |
|------|------|
| Real-time preview - instant feedback | Need to verify frontend actually performed claimed edits |
| Lower server load - client handles processing | ZK proof complexity increases |
| Better UX | Browser compatibility issues |
| Lower bandwidth - edited image not uploaded | Trust model shifted to client |

### Decision: Backend API (Recommended for v1)

**Rationale:**
- Simpler ZK proof generation - backend knows exact operations
- Clear trust model - server controls editing operations
- Easier to implement correctly first, then optimize

**Future consideration:** After v1, can add frontend editing with additional ZK circuits to prove correct execution.

## Architecture Decision: Go API vs Rust API

### Option A: Go API Server + Rust Core

```
Frontend → Go API (HTTP/gRPC) → Rust Editor (Photon) → Edited Image
```

| Pros | Cons |
|------|------|
| Follows project convention (Go for API, Rust for core) | Requires IPC/FFI overhead |
| Go HTTP/gRPC ecosystem is mature | Additional glue code needed |
| Easy to scale API layer independently | |
| Clean separation of concerns | |

### Option B: Rust API Server (Full stack Rust)

```
Frontend → Rust HTTP Server → Editor Module → Edited Image
```

| Pros | Cons |
|------|------|
| No IPC overhead - direct function calls | Less mature HTTP ecosystem in Rust |
| Shared type system | More complex to set up |
| Best performance | |
| Simpler deployment (single binary) | |
| Lower complexity for v1 | |

### Decision: Rust API Server (v1)

**Rationale:**
- Simpler system - no IPC/FFI complexity
- Single binary deployment
- Faster to implement for v1
- Can add Go API layer later if needed

## What Changes
- **NEW**: Add `editing` capability as REST API
- Accept original image from Capture & Provenance Layer
- Support mandatory Crop operation via API
- Support Resize operation via API
- Support optional operations: rotation, filters
- Output edited image + editing record (operation type + parameters)
- Pass edited image + editing record to ZK Proof Layer

## Impact
- Affected specs: New capability `editing`
- Affected code: Rust module under `src/editor/` (includes HTTP API)
- Dependencies: Photon library (Rust image processing), Axum or Actix (Rust HTTP)
