//! User model with validation.
//!
//! TAINT SOURCE: All Create/Update DTOs receive data from HTTP requests.

use serde::{Deserialize, Serialize};
use validator::Validate;

/// User entity from database
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub username: String,
    pub height_cm: Option<f64>,
    pub weight_kg: Option<f64>,
    pub age: Option<i32>,
    pub daily_calorie_goal: i32,
    pub created_at: String,
    pub updated_at: String,
}

/// User response (without password)
#[derive(Debug, Serialize, Clone)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub username: String,
    pub height_cm: Option<f64>,
    pub weight_kg: Option<f64>,
    pub age: Option<i32>,
    pub daily_calorie_goal: i32,
    pub created_at: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            username: user.username,
            height_cm: user.height_cm,
            weight_kg: user.weight_kg,
            age: user.age,
            daily_calorie_goal: user.daily_calorie_goal,
            created_at: user.created_at,
        }
    }
}

/// TAINT SOURCE: Registration request from HTTP
/// Validates like Joi/Zod schema
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email(message = "Invalid email format"))]
    #[validate(length(max = 255, message = "Email too long"))]
    pub email: String,

    #[validate(length(min = 8, max = 128, message = "Password must be 8-128 characters"))]
    pub password: String,

    #[validate(length(min = 3, max = 50, message = "Username must be 3-50 characters"))]
    pub username: String,

    pub height_cm: Option<f64>,
    pub weight_kg: Option<f64>,
    pub age: Option<i32>,

    #[validate(range(min = 500, max = 10000, message = "Calorie goal must be 500-10000"))]
    pub daily_calorie_goal: Option<i32>,
}

/// TAINT SOURCE: Update profile request from HTTP
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 3, max = 50, message = "Username must be 3-50 characters"))]
    pub username: Option<String>,

    pub height_cm: Option<f64>,
    pub weight_kg: Option<f64>,
    pub age: Option<i32>,

    #[validate(range(min = 500, max = 10000, message = "Calorie goal must be 500-10000"))]
    pub daily_calorie_goal: Option<i32>,
}

/// TAINT SOURCE: Login request from HTTP
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password required"))]
    pub password: String,
}

/// Login response with JWT token
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
    pub expires_at: String,
}

/// Password change request
#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 1, message = "Current password required"))]
    pub current_password: String,

    #[validate(length(min = 8, max = 128, message = "New password must be 8-128 characters"))]
    pub new_password: String,
}

/// JWT claims for authentication
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub email: String,
    pub exp: i64,
    pub iat: i64,
}
