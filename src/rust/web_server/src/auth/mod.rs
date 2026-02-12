use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use utoipa::ToSchema;

/// JWT secret key - In production, this should be an environment variable
const SECRET: &[u8] = b"your-secret-key-change-this-in-production";

/// Token claims structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,  // Subject (user identifier)
    pub exp: usize,   // Expiration time
    pub iat: usize,   // Issued at
}

impl Claims {
    /// Create a new Claims instance with a 24-hour expiration
    pub fn new(user_id: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        
        Self {
            sub: user_id,
            exp: now + 86400, // 24 hours
            iat: now,
        }
    }
}

/// Generate a JWT token
pub fn generate_token(user_id: String) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(user_id);
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
}

/// Validate a JWT token
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

/// Authentication middleware
pub async fn auth_middleware(mut request: Request, next: Next) -> Result<Response, StatusCode> {
    // Get the authorization header
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    // Check if the header is present and valid
    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    // Validate the token
    match validate_token(token) {
        Ok(claims) => {
            // Insert claims into request extensions so handlers can access them
            request.extensions_mut().insert(claims);
            Ok(next.run(request).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

/// Login request structure
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Login response structure
#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: String,
}

/// Simple authentication handler (for demonstration)
/// In production, you would verify credentials against a database
pub async fn login(
    axum::Json(payload): axum::Json<LoginRequest>,
) -> Result<axum::Json<LoginResponse>, StatusCode> {
    // For demonstration, we accept any username/password combination
    // In production, verify against a database with hashed passwords
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Generate a token
    let token = generate_token(payload.username.clone())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(axum::Json(LoginResponse {
        token,
        user_id: payload.username,
    }))
}
