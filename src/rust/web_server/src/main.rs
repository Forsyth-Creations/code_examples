mod auth;
mod routers;

use axum::{
    middleware,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use auth::{login, LoginRequest, LoginResponse};

/// Main API documentation combining all module docs
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Rust Web Server API",
        version = "1.0.0",
        description = "A FastAPI-like web server built with Rust, featuring authentication, OpenAPI documentation, and modular routers"
    ),
    paths(),
    components(schemas(LoginRequest, LoginResponse)),
    modifiers(&SecurityAddon),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "items", description = "Item management endpoints")
    )
)]
struct ApiDoc;

/// Add security scheme to OpenAPI
struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
        
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

/// Merge OpenAPI docs from different modules
fn merge_openapi_docs() -> utoipa::openapi::OpenApi {
    let mut main_doc = ApiDoc::openapi();
    
    // Merge users documentation
    let users_doc = routers::users::UsersApiDoc::openapi();
    main_doc.paths.paths.extend(users_doc.paths.paths);
    if let Some(components) = users_doc.components {
        if let Some(main_components) = &mut main_doc.components {
            main_components.schemas.extend(components.schemas);
        }
    }
    
    // Merge items documentation
    let items_doc = routers::items::ItemsApiDoc::openapi();
    main_doc.paths.paths.extend(items_doc.paths.paths);
    if let Some(components) = items_doc.components {
        if let Some(main_components) = &mut main_doc.components {
            main_components.schemas.extend(components.schemas);
        }
    }
    
    main_doc
}

/// Health check endpoint
async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "version": "1.0.0"
    }))
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // Create CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build the application with routes
    let app = Router::new()
        // Health check
        .route("/health", get(health_check))
        
        // Authentication routes (public)
        .route("/api/auth/login", post(login))
        
        // Public routes
        .nest("/api/items", routers::items::router())
        
        // Protected routes (require authentication)
        .nest(
            "/api/users",
            routers::users::router().layer(middleware::from_fn(auth::auth_middleware)),
        )
        .nest(
            "/api/items",
            routers::items::authenticated_router()
                .layer(middleware::from_fn(auth::auth_middleware)),
        )
        
        // Swagger UI and OpenAPI JSON
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", merge_openapi_docs()))
        
        // Add middleware
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");
    
    println!("🚀 Server running on http://0.0.0.0:3000");
    println!("📚 Swagger UI available at http://0.0.0.0:3000/swagger-ui");
    println!("📄 OpenAPI JSON available at http://0.0.0.0:3000/api-docs/openapi.json");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

