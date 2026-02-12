# Rust Web Server

A FastAPI-like web server built with Rust, featuring:

- **Authentication**: JWT-based authentication with Bearer tokens
- **OpenAPI Documentation**: Automatic OpenAPI/Swagger documentation
- **Modular Routers**: Discrete router modules that can be easily added to the main server
- **Type Safety**: Fully type-safe with Rust's type system
- **Modern Stack**: Built with Axum, Tokio, and Utoipa

## Features

### 🔐 Authentication

The server includes JWT-based authentication with:
- Login endpoint to obtain a token
- Authentication middleware for protected routes
- Bearer token validation

### 📚 OpenAPI Documentation

- Swagger UI available at `/swagger-ui`
- OpenAPI JSON schema at `/api-docs/openapi.json`
- Automatically generated from code annotations

### 🛣️ Modular Routers

The server uses a modular router architecture similar to FastAPI:

- **Users Router** (`routers/users.rs`): User management endpoints (protected)
- **Items Router** (`routers/items.rs`): Item management endpoints (mixed public/protected)

Each router module:
- Defines its own OpenAPI documentation
- Can be independently developed and tested
- Is easily added to the main server

## Running the Server

```bash
# Build and run
cargo run

# Or with release optimizations
cargo run --release
```

The server will start on `http://localhost:3000`

## API Endpoints

### Authentication

#### POST `/api/auth/login`
Login to get a JWT token.

**Request:**
```json
{
  "username": "alice",
  "password": "password123"
}
```

**Response:**
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGc...",
  "user_id": "alice"
}
```

### Users (Protected Routes)

All user endpoints require authentication via Bearer token.

#### GET `/api/users`
Get all users.

**Headers:**
```
Authorization: Bearer <your-token>
```

**Query Parameters:**
- `limit` (optional): Maximum number of users to return
- `offset` (optional): Number of users to skip

#### GET `/api/users/{id}`
Get a specific user by ID.

#### POST `/api/users`
Create a new user.

**Request:**
```json
{
  "username": "newuser",
  "email": "newuser@example.com"
}
```

### Items (Mixed Routes)

#### GET `/api/items` (Public)
Get all items.

**Query Parameters:**
- `limit` (optional): Maximum number of items to return
- `offset` (optional): Number of items to skip
- `min_price` (optional): Minimum price filter
- `max_price` (optional): Maximum price filter

#### GET `/api/items/{id}` (Public)
Get a specific item by ID.

#### POST `/api/items` (Protected)
Create a new item (requires authentication).

**Request:**
```json
{
  "name": "New Widget",
  "description": "A brand new widget",
  "price": 39.99
}
```

#### PUT `/api/items/{id}` (Protected)
Update an existing item (requires authentication).

#### DELETE `/api/items/{id}` (Protected)
Delete an item (requires authentication).

## Example Usage

### 1. Login to get a token

```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "alice", "password": "password"}'
```

### 2. Use the token to access protected endpoints

```bash
# Get users (requires authentication)
curl http://localhost:3000/api/users \
  -H "Authorization: Bearer <your-token>"

# Get items (public)
curl http://localhost:3000/api/items

# Create an item (requires authentication)
curl -X POST http://localhost:3000/api/items \
  -H "Authorization: Bearer <your-token>" \
  -H "Content-Type: application/json" \
  -d '{"name": "New Item", "description": "Description", "price": 29.99}'
```

### 3. View API Documentation

Open your browser and navigate to:
- Swagger UI: http://localhost:3000/swagger-ui
- OpenAPI JSON: http://localhost:3000/api-docs/openapi.json

## Project Structure

```
web_server/
├── Cargo.toml          # Dependencies and project configuration
├── src/
│   ├── main.rs         # Main server setup, routing, and OpenAPI merging
│   ├── auth/
│   │   └── mod.rs      # Authentication logic (JWT, middleware)
│   └── routers/
│       ├── mod.rs      # Router module exports
│       ├── users.rs    # Users router with CRUD operations
│       └── items.rs    # Items router with CRUD operations
└── README.md           # This file
```

## Adding a New Router

To add a new router module:

1. Create a new file in `src/routers/` (e.g., `posts.rs`)
2. Define your routes and handlers with `#[utoipa::path]` annotations
3. Create an OpenAPI doc struct with `#[derive(OpenApi)]`
4. Export the module in `src/routers/mod.rs`
5. Add the router to `main.rs`:
   ```rust
   // In main.rs
   .nest("/api/posts", routers::posts::router())
   
   // And merge the OpenAPI docs in merge_openapi_docs()
   let posts_doc = routers::posts::PostsApiDoc::openapi();
   // ... merge logic
   ```

## Dependencies

Key dependencies used:

- **axum** - Web framework
- **tokio** - Async runtime
- **utoipa** - OpenAPI documentation generation
- **utoipa-swagger-ui** - Swagger UI integration
- **jsonwebtoken** - JWT authentication
- **serde** - Serialization/deserialization
- **tower-http** - HTTP middleware (CORS, tracing)
- **tracing** - Logging

## Security Notes

⚠️ **Important**: This is a demonstration project. For production use:

1. Use environment variables for secrets (JWT key, database credentials)
2. Implement proper password hashing with bcrypt
3. Use a real database instead of in-memory data
4. Add rate limiting
5. Implement proper error handling
6. Use HTTPS
7. Add input validation
8. Implement CSRF protection
9. Set appropriate CORS policies

## License

This is example code for educational purposes.
