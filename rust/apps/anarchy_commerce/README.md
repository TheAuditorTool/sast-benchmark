# Anarchy Commerce

A realistic polyglot e-commerce monorepo for SAST/taint analysis testing.

## Architecture

```
anarchy_commerce/
├── web/                      # React frontend (TypeScript)
├── services/
│   ├── gateway/              # Node.js API Gateway (BFF)
│   ├── users/                # Python/FastAPI user service
│   ├── payments/             # Rust payment processing (security-critical)
│   ├── search/               # Rust search service (performance-critical)
│   └── recommendations/      # Python ML recommendations
├── workers/
│   ├── email-sender/         # Python Celery worker
│   └── image-processor/      # Rust image processing
├── shared/
│   ├── types/                # TypeScript shared types
│   ├── python/               # Python shared utilities
│   └── proto/                # gRPC/Protobuf definitions
└── infrastructure/           # Docker, K8s configs
```

## Language Choices (Realistic Reasons)

| Service | Language | Why |
|---------|----------|-----|
| Web | TypeScript/React | Industry standard for SPAs |
| Gateway | Node.js | Frontend team can contribute, fast iteration |
| Users | Python/FastAPI | Team expertise, rapid development |
| Payments | Rust | Security-critical, memory safety, performance |
| Search | Rust | Performance-critical, low latency required |
| Recommendations | Python | ML models, scikit-learn/PyTorch ecosystem |
| Email Worker | Python | Simple task, team knows Python |
| Image Processor | Rust | CPU-intensive, performance matters |

## Data Flow

```
User Browser
     │
     ▼
┌─────────────────┐
│   Web (React)   │  localhost:3000
└────────┬────────┘
         │ HTTP/REST
         ▼
┌─────────────────┐
│ Gateway (Node)  │  localhost:4000
└────────┬────────┘
         │ Internal HTTP/gRPC
         ▼
    ┌────┴────┐
    │         │
    ▼         ▼
┌───────┐ ┌────────┐ ┌───────────────┐
│ Users │ │Payments│ │Recommendations│
│(Python)│ │ (Rust) │ │   (Python)    │
└───────┘ └────────┘ └───────────────┘
    │         │              │
    └─────────┴──────────────┘
              │
              ▼
         [PostgreSQL]
```

## Running Locally

```bash
# Install all dependencies
npm install                          # Node workspaces
poetry install                       # Python services
cargo build                          # Rust services

# Start services
npm run dev -w web                   # Frontend on :3000
npm run dev -w services/gateway      # Gateway on :4000
cd services/users && uvicorn main:app --port 4001
cd services/payments && cargo run    # Payments on :4002
cd services/search && cargo run      # Search on :4003
```

## Cross-Boundary Taint Flows

This monorepo has REAL data flows between services:

1. **User Registration**: Web → Gateway → Users (Python) → DB
2. **Payment Processing**: Web → Gateway → Payments (Rust) → Stripe API
3. **Product Search**: Web → Gateway → Search (Rust) → Elasticsearch
4. **Recommendations**: Web → Gateway → Recommendations (Python) → ML Model

Each flow crosses language boundaries with actual HTTP/gRPC calls.
