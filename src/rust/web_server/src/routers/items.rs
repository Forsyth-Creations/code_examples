use axum::{
    extract::{Path, Query},
    http::StatusCode,
    routing::{get, post, put},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

use crate::auth::Claims;

/// Item model
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: f64,
}

/// Create item request
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateItemRequest {
    pub name: String,
    pub description: String,
    pub price: f64,
}

/// Update item request
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateItemRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
}

/// Query parameters for listing items
#[derive(Debug, Deserialize, ToSchema)]
pub struct ListItemsQuery {
    #[serde(default)]
    pub limit: Option<usize>,
    #[serde(default)]
    pub offset: Option<usize>,
    #[serde(default)]
    pub min_price: Option<f64>,
    #[serde(default)]
    pub max_price: Option<f64>,
}

/// Get all items (public endpoint)
#[utoipa::path(
    get,
    path = "/api/items",
    responses(
        (status = 200, description = "List of items", body = Vec<Item>)
    ),
    params(
        ("limit" = Option<usize>, Query, description = "Maximum number of items to return"),
        ("offset" = Option<usize>, Query, description = "Number of items to skip"),
        ("min_price" = Option<f64>, Query, description = "Minimum price filter"),
        ("max_price" = Option<f64>, Query, description = "Maximum price filter")
    )
)]
async fn get_items(Query(params): Query<ListItemsQuery>) -> Json<Vec<Item>> {
    // In a real application, fetch from database
    let mut items = vec![
        Item {
            id: "1".to_string(),
            name: "Widget".to_string(),
            description: "A useful widget".to_string(),
            price: 29.99,
        },
        Item {
            id: "2".to_string(),
            name: "Gadget".to_string(),
            description: "An amazing gadget".to_string(),
            price: 49.99,
        },
        Item {
            id: "3".to_string(),
            name: "Doohickey".to_string(),
            description: "A mysterious doohickey".to_string(),
            price: 19.99,
        },
    ];

    // Apply price filters
    if let Some(min_price) = params.min_price {
        items.retain(|item| item.price >= min_price);
    }
    if let Some(max_price) = params.max_price {
        items.retain(|item| item.price <= max_price);
    }

    // Apply pagination
    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(10);
    let result: Vec<Item> = items.into_iter().skip(offset).take(limit).collect();

    Json(result)
}

/// Get a specific item by ID (public endpoint)
#[utoipa::path(
    get,
    path = "/api/items/{id}",
    responses(
        (status = 200, description = "Item found", body = Item),
        (status = 404, description = "Item not found")
    ),
    params(
        ("id" = String, Path, description = "Item ID")
    )
)]
async fn get_item_by_id(Path(id): Path<String>) -> Result<Json<Item>, StatusCode> {
    // In a real application, fetch from database
    if id == "1" {
        Ok(Json(Item {
            id: "1".to_string(),
            name: "Widget".to_string(),
            description: "A useful widget".to_string(),
            price: 29.99,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Create a new item (requires authentication)
#[utoipa::path(
    post,
    path = "/api/items",
    request_body = CreateItemRequest,
    responses(
        (status = 201, description = "Item created", body = Item),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
async fn create_item(
    Extension(_claims): Extension<Claims>,
    Json(payload): Json<CreateItemRequest>,
) -> Result<(StatusCode, Json<Item>), StatusCode> {
    // In a real application, save to database
    let item = Item {
        id: "new_id".to_string(),
        name: payload.name,
        description: payload.description,
        price: payload.price,
    };

    Ok((StatusCode::CREATED, Json(item)))
}

/// Update an existing item (requires authentication)
#[utoipa::path(
    put,
    path = "/api/items/{id}",
    request_body = UpdateItemRequest,
    responses(
        (status = 200, description = "Item updated", body = Item),
        (status = 404, description = "Item not found"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Item ID")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
async fn update_item(
    Extension(_claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateItemRequest>,
) -> Result<Json<Item>, StatusCode> {
    // In a real application, update in database
    if id == "1" {
        let item = Item {
            id: "1".to_string(),
            name: payload.name.unwrap_or_else(|| "Widget".to_string()),
            description: payload
                .description
                .unwrap_or_else(|| "A useful widget".to_string()),
            price: payload.price.unwrap_or(29.99),
        };
        Ok(Json(item))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Delete an item (requires authentication)
#[utoipa::path(
    delete,
    path = "/api/items/{id}",
    responses(
        (status = 204, description = "Item deleted"),
        (status = 404, description = "Item not found"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("id" = String, Path, description = "Item ID")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
async fn delete_item(
    Extension(_claims): Extension<Claims>,
    Path(_id): Path<String>,
) -> StatusCode {
    // In a real application, delete from database
    StatusCode::NO_CONTENT
}

/// Create the items router with public and authenticated endpoints
pub fn router() -> Router {
    Router::new()
        .route("/", get(get_items))
        .route("/:id", get(get_item_by_id))
}

/// Create the authenticated items router
pub fn authenticated_router() -> Router {
    Router::new()
        .route("/", post(create_item))
        .route("/:id", put(update_item).delete(delete_item))
}

/// OpenAPI documentation for items module
#[derive(OpenApi)]
#[openapi(
    paths(
        get_items,
        get_item_by_id,
        create_item,
        update_item,
        delete_item
    ),
    components(schemas(Item, CreateItemRequest, UpdateItemRequest, ListItemsQuery))
)]
pub struct ItemsApiDoc;
