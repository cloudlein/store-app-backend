# Backend Development Roadmap: Nagi Store API

This document outlines the development tasks for the Nagi Store backend API, strictly limited to the features currently implemented in the frontend application, with a focus on clean architecture and specialized logging.

## Technical Stack
- **Language**: Rust
- **Web Framework**: Axum
- **Database**: PostgreSQL
- **Database Driver/ORM**: SQLx
- **Serialization**: Serde
- **Logging**: Tracing & Tracing-subscriber
- **Documentation**: Utoipa (Swagger UI)

---

## Clean Architecture Directory Structure
To maintain a clean and scalable codebase, the following structure is recommended:

```text
store-app-backend/
├── migrations/          # SQLx database migrations
├── src/
│   ├── main.rs          # Application entry point and server setup
│   ├── config/          # Database connection and environment config
│   ├── handlers/        # API route handlers (Logic for each endpoint)
│   ├── models/          # Data structures (DB models and API DTOs)
│   ├── routes/          # Route definitions and mapping to handlers
│   ├── middleware/      # Logging, CORS, and other Axum layers
│   ├── utils/           # Shared utilities (Response wrappers, constants)
│   └── docs/            # Swagger/OpenAPI specifications
└── Cargo.toml
```

---

## API Specification Standards

### Standardized Response Format
```json
{
  "success": true,
  "message": "Operation successful",
  "data": { ... },
  "error": null
}
```

---

## Development Phases

### Phase 1: Foundation, Data Model, and Logging System
- [ ] **Initialize Rust Project**: Setup Axum, SQLx, and Serde.
- [ ] **Database Schema**:
    - `products`: id, title, price, description, image_url.
    - `orders`: id, total_price, items (JSONB), created_at.
- [ ] **Advanced Logging Implementation**:
    - [ ] Configure `tracing-subscriber` for structured console logging.
    - [ ] Implement middleware to capture HTTP requests and responses.
    - [ ] Setup file-based rotation logging.

### Phase 2: Product API
- [ ] **GET /api/v1/products**: Retrieve products for the main page.
    - Implementation location: `handlers/product.rs`, `models/product.rs`.

### Phase 3: Order API
- [ ] **POST /api/v1/orders**: Create a new order from cart data.
    - Implementation location: `handlers/order.rs`, `models/order.rs`.

### Phase 4: Documentation (Swagger/OpenAPI)
- [ ] **Utoipa Integration**: Implement Swagger UI to visualize and test the API.
    - [ ] Define OpenAPI schemas for Products and Orders.
    - [ ] Expose Swagger UI at `/swagger-ui`.
    - Implementation location: `docs/mod.rs` or `main.rs`.

### Phase 5: Integration and Health
- [ ] **CORS Configuration**: Allow requests from the frontend origin.
- [ ] **Health Check**: `GET /health` to verify service status.

---

## Integration Guidelines for Frontend
- Update the `axios` call in `App.jsx` to the local backend endpoint `/api/v1/products`.
- Implement the `onClick` handler for the "Checkout" button to send cart data to `/api/v1/orders`.
