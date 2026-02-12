use axum::{
    extract::{Path, Query},
    http::StatusCode,
    routing::get,
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

use crate::auth::Claims;

/// User model
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
}

/// Create user request
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
}

/// Query parameters for listing users
#[derive(Debug, Deserialize, ToSchema)]
pub struct ListUsersQuery {
    #[serde(default)]
    pub limit: Option<usize>,
    #[serde(default)]
    pub offset: Option<usize>,
}

/// Get all users (requires authentication)
#[utoipa::path(
    get,
    path = "/api/users",
    responses(
        (status = 200, description = "List of users", body = Vec<User>),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("limit" = Option<usize>, Query, description = "Maximum number of users to return"),
        ("offset" = Option<usize>, Query, description = "Number of users to skip")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
async fn get_users(
    Extension(_claims): Extension<Claims>,
    Query(params): Query<ListUsersQuery>,
) -> Json<Vec<User>> {
    // In a real application, fetch from database
    let users = vec![
        User {
            id: "1".to_string(),
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: "2".to_string(),
            username: "bob".to_string(),
            email: "bob@example.com".to_string(),
        },
    ];

    // Apply pagination
    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(10);
    let result: Vec<User> = users.into_iter().skip(offset).take(limit).collect();

    Json(result)
}

/// Get a specific user by ID (requires authentication)
#[utoipa::path(
    get,
    path = "/api/users/{id}",
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "User ID")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
async fn get_user_by_id(
    Extension(_claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<User>, StatusCode> {
    // In a real application, fetch from database
    if id == "1" {
        Ok(Json(User {
            id: "1".to_string(),
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Create a new user (requires authentication)
#[utoipa::path(
    post,
    path = "/api/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created", body = User),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
async fn create_user(
    Extension(_claims): Extension<Claims>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    // In a real application, save to database
    let user = User {
        id: "new_id".to_string(),
        username: payload.username,
        email: payload.email,
    };

    Ok((StatusCode::CREATED, Json(user)))
}

/// Create the users router
pub fn router() -> Router {
    Router::new()
        .route("/", get(get_users).post(create_user))
        .route("/:id", get(get_user_by_id))
}

/// OpenAPI documentation for users module
#[derive(OpenApi)]
#[openapi(
    paths(get_users, get_user_by_id, create_user),
    components(schemas(User, CreateUserRequest, ListUsersQuery))
)]
pub struct UsersApiDoc;
