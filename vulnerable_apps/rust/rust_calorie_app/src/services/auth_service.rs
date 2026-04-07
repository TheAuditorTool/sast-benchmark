//! Authentication service - user registration, login, JWT handling.
//!
//! TAINT TRANSFORM: HTTP credentials -> hash/token -> database storage

use crate::config::AppConfig;
use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::{
    Claims, CreateUserRequest, LoginRequest, LoginResponse, User, UserResponse,
};
use crate::repositories::UserRepository;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

/// Authentication service
pub struct AuthService;

impl AuthService {
    /// Register a new user.
    ///
    /// TAINT TRANSFORM: password (HTTP) -> bcrypt hash -> database
    /// Multi-hop: CreateUserRequest -> hash_password -> UserRepository::create -> SQL
    pub async fn register(
        pool: &DbPool,
        request: CreateUserRequest,
    ) -> Result<UserResponse, AppError> {
        // Check if email already exists
        if UserRepository::email_exists(pool, &request.email).await? {
            return Err(AppError::Conflict("Email already registered".to_string()));
        }

        // TAINT TRANSFORM: Hash the password (HTTP input -> bcrypt)
        let password_hash = Self::hash_password(&request.password)?;

        // Create user with hashed password
        // TAINT FLOW: request (HTTP) + password_hash -> UserRepository -> SQL
        let user = UserRepository::create(pool, &request, &password_hash).await?;

        Ok(UserResponse::from(user))
    }

    /// Authenticate user and return JWT token.
    ///
    /// TAINT TRANSFORM: email/password (HTTP) -> verify -> JWT token
    pub async fn login(
        pool: &DbPool,
        config: &AppConfig,
        request: LoginRequest,
    ) -> Result<LoginResponse, AppError> {
        // TAINT FLOW: email from HTTP -> UserRepository::find_by_email -> SQL WHERE
        let user = UserRepository::find_by_email(pool, &request.email)
            .await?
            .ok_or(AppError::Unauthorized("Invalid credentials".to_string()))?;

        // TAINT TRANSFORM: Verify password (HTTP input vs stored hash)
        if !Self::verify_password(&request.password, &user.password_hash)? {
            return Err(AppError::Unauthorized("Invalid credentials".to_string()));
        }

        // Generate JWT token
        let token = Self::generate_token(config, &user)?;

        let expires_at = Utc::now() + Duration::hours(config.jwt_expiration_hours);

        Ok(LoginResponse {
            token,
            user: UserResponse::from(user),
            expires_at: expires_at.to_rfc3339(),
        })
    }

    /// Verify a JWT token and return claims.
    ///
    /// TAINT SOURCE: Token from Authorization header -> decoded claims
    pub fn verify_token(config: &AppConfig, token: &str) -> Result<Claims, AppError> {
        // TAINT: token from HTTP Authorization header
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))?;

        Ok(token_data.claims)
    }

    /// Get user by ID (for middleware).
    pub async fn get_user_by_id(pool: &DbPool, user_id: &str) -> Result<User, AppError> {
        UserRepository::find_by_id(pool, user_id)
            .await?
            .ok_or(AppError::NotFound("User not found".to_string()))
    }

    /// Change user password.
    ///
    /// TAINT TRANSFORM: old + new password (HTTP) -> verify + hash -> database
    pub async fn change_password(
        pool: &DbPool,
        user_id: &str,
        current_password: &str,
        new_password: &str,
    ) -> Result<(), AppError> {
        // Get current user
        let user = UserRepository::find_by_id(pool, user_id)
            .await?
            .ok_or(AppError::NotFound("User not found".to_string()))?;

        // Verify current password
        // TAINT: current_password from HTTP
        if !Self::verify_password(current_password, &user.password_hash)? {
            return Err(AppError::Unauthorized("Current password is incorrect".to_string()));
        }

        // Hash new password
        // TAINT TRANSFORM: new_password (HTTP) -> bcrypt hash
        let new_hash = Self::hash_password(new_password)?;

        // Update in database
        // TAINT FLOW: new_hash -> UserRepository::update_password -> SQL UPDATE
        UserRepository::update_password(pool, user_id, &new_hash).await?;

        Ok(())
    }

    // ==================== Private Helpers ====================

    /// Hash a password using bcrypt.
    ///
    /// TAINT TRANSFORM: plaintext -> bcrypt hash
    fn hash_password(password: &str) -> Result<String, AppError> {
        // TAINT: password from HTTP input
        hash(password, DEFAULT_COST).map_err(|e| AppError::Internal(format!("Hash error: {}", e)))
    }

    /// Verify a password against a hash.
    fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
        // TAINT: password from HTTP input
        verify(password, hash).map_err(|e| AppError::Internal(format!("Verify error: {}", e)))
    }

    /// Generate a JWT token for a user.
    fn generate_token(config: &AppConfig, user: &User) -> Result<String, AppError> {
        let expiration = Utc::now() + Duration::hours(config.jwt_expiration_hours);

        let claims = Claims {
            sub: user.id.clone(),
            email: user.email.clone(),
            exp: expiration.timestamp(),
            iat: Utc::now().timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::Internal(format!("Token generation error: {}", e)))
    }
}
