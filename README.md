# Web Nexus CMS - Rust Workspace

**Secure, contract-driven, Rust-to-WASM content management system**

## Architecture Overview

```
web-nexus-cms/
├── contracts/    - Shared types & API contracts (Layer 0)
├── state/        - CRDT state management (Layer 2)
├── api/          - REST/GraphQL API (Cloudflare Workers)
├── edge/         - Durable Objects coordination
├── cms/          - Admin portal (Leptos WASM)
└── website/      - Public sites (Leptos WASM)
    ├── mikeandthemonsters/  - First deployment
    └── [future sites...]
```

## Crates

### contracts
Shared Rust contracts for all crates. Auto-generates TypeScript.
- User & role types
- Content schemas (shows, songs, photos, videos)
- API contracts (OpenAPI)

### state
CRDT-based state management with edge sync.
- Local-first state (CRDT merge)
- Edge state (Durable Objects)
- Offline support

### api
Cloudflare Workers API layer.
- Authentication & authorization
- CRUD operations
- WebSocket collaboration

### edge
Durable Objects for edge state coordination.
- Project state coordination
- Real-time collaboration
- Conflict resolution

### cms
Admin portal compiled to WASM.
- Visual content editors
- Role-based access
- Real-time collaboration

### website
Public sites compiled to WASM.
- mikeandthemonsters (first deployment)
- Future: more band/event sites

## Development

```bash
# Build all crates
cargo build --release

# Run tests
cargo test

# Build WASM for website
cd website/mikeandthemonsters
trunk build --release

# Run locally
cargo run --bin api-dev
```

## Deployment

Each crate has its own deployment target:

- **api**: Cloudflare Workers
- **cms**: Cloudflare Pages (admin.domain.com)
- **website**: Cloudflare Pages (domain.com)

## Security

- ✅ Rust memory safety
- ✅ WASM sandboxing
- ✅ Contract-driven types
- ✅ Zero-trust architecture

Based on [WEB_NEXUS V2_ARCHITECTURE](./docs/WEB_NEXUS_V2_ARCHITECTURE.md)
