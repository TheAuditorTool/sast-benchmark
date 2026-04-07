//! User repository - SQL queries for user management.
//!
//! TAINT SINK: All methods receive data that originated from HTTP requests.
//! Multi-hop flow: HTTP -> validation -> service -> repository -> SQL

use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::{CreateUserRequest, UpdateUserRequest, User};
use sqlx::Row;
use uuid::Uuid;

/// Repository for user database operations
pub struct UserRepository;

impl UserRepository {
    // vuln-code-snippet start sqliCalorieCreateUser
    /// Create a new user.
    ///
    /// TAINT SINK: email, password_hash, username flow from HTTP request
    /// Multi-hop: CreateUserRequest -> AuthService::register -> here
    pub async fn create(
        pool: &DbPool,
        request: &CreateUserRequest,
        password_hash: &str,
    ) -> Result<User, AppError> {
        let id = Uuid::new_v4().to_string();
        let daily_goal = request.daily_calorie_goal.unwrap_or(2000);

        // TAINT SINK: sqlx::query with tainted data
        sqlx::query(
            r#"
            INSERT INTO users (id, email, password_hash, username, height_cm, weight_kg, age, daily_calorie_goal, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))
            "#,
        )
        // vuln-code-snippet target-line sqliCalorieCreateUser
        .bind(&id)
        .bind(&request.email)      // TAINT: from HTTP
        .bind(password_hash)        // TAINT: derived from HTTP password
        .bind(&request.username)    // TAINT: from HTTP
        .bind(request.height_cm)    // TAINT: from HTTP
        .bind(request.weight_kg)    // TAINT: from HTTP
        .bind(request.age)          // TAINT: from HTTP
        .bind(daily_goal)           // TAINT: from HTTP
        .execute(pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("UNIQUE constraint failed") {
                AppError::Conflict("Email already registered".to_string())
            } else {
                AppError::Database(e.to_string())
            }
        })?;

        Self::find_by_id(pool, &id).await?.ok_or(AppError::NotFound("User not found".to_string()))
    }
    // vuln-code-snippet end sqliCalorieCreateUser

    /// Find user by ID.
    pub async fn find_by_id(pool: &DbPool, id: &str) -> Result<Option<User>, AppError> {
        // TAINT SINK: id parameter in WHERE clause
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, username, height_cm, weight_kg, age, daily_calorie_goal, created_at, updated_at FROM users WHERE id = ?",
        )
        .bind(id)  // TAINT: could be from path param
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(user)
    }

    // vuln-code-snippet start sqliCalorieFindByEmail
    /// Find user by email.
    ///
    /// TAINT SINK: email from login request flows to WHERE clause
    pub async fn find_by_email(pool: &DbPool, email: &str) -> Result<Option<User>, AppError> {
        // TAINT SINK: email in WHERE clause
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, username, height_cm, weight_kg, age, daily_calorie_goal, created_at, updated_at FROM users WHERE email = ?",
        )
        // vuln-code-snippet target-line sqliCalorieFindByEmail
        .bind(email)  // TAINT: from login form
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(user)
    }
    // vuln-code-snippet end sqliCalorieFindByEmail

    // vuln-code-snippet start sqliCalorieUpdateUser
    /// Update user profile.
    ///
    /// TAINT SINK: UpdateUserRequest fields flow to UPDATE statement
    pub async fn update(
        pool: &DbPool,
        user_id: &str,
        request: &UpdateUserRequest,
    ) -> Result<User, AppError> {
        // Build dynamic update - collect field updates
        let mut set_clauses = Vec::new();

        if request.username.is_some() {
            set_clauses.push("username = ?");
        }
        if request.height_cm.is_some() {
            set_clauses.push("height_cm = ?");
        }
        if request.weight_kg.is_some() {
            set_clauses.push("weight_kg = ?");
        }
        if request.age.is_some() {
            set_clauses.push("age = ?");
        }
        if request.daily_calorie_goal.is_some() {
            set_clauses.push("daily_calorie_goal = ?");
        }

        if set_clauses.is_empty() {
            return Self::find_by_id(pool, user_id)
                .await?
                .ok_or(AppError::NotFound("User not found".to_string()));
        }

        set_clauses.push("updated_at = datetime('now')");

        let query = format!(
            "UPDATE users SET {} WHERE id = ?",
            set_clauses.join(", ")
        );

        // Build query with dynamic bindings
        let mut q = sqlx::query(&query);

        if let Some(ref username) = request.username {
            // vuln-code-snippet target-line sqliCalorieUpdateUser
            q = q.bind(username);  // TAINT: from HTTP
        }
        if let Some(height) = request.height_cm {
            q = q.bind(height);  // TAINT: from HTTP
        }
        if let Some(weight) = request.weight_kg {
            q = q.bind(weight);  // TAINT: from HTTP
        }
        if let Some(age) = request.age {
            q = q.bind(age);  // TAINT: from HTTP
        }
        if let Some(goal) = request.daily_calorie_goal {
            q = q.bind(goal);  // TAINT: from HTTP
        }
        q = q.bind(user_id);

        q.execute(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Self::find_by_id(pool, user_id)
            .await?
            .ok_or(AppError::NotFound("User not found".to_string()))
    }
    // vuln-code-snippet end sqliCalorieUpdateUser

    /// Update user password.
    ///
    /// TAINT SINK: new password hash flows to UPDATE
    pub async fn update_password(
        pool: &DbPool,
        user_id: &str,
        new_password_hash: &str,
    ) -> Result<(), AppError> {
        // TAINT SINK: password_hash derived from HTTP input
        sqlx::query(
            "UPDATE users SET password_hash = ?, updated_at = datetime('now') WHERE id = ?",
        )
        .bind(new_password_hash)  // TAINT: derived from HTTP password
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    /// Delete user account.
    pub async fn delete(pool: &DbPool, user_id: &str) -> Result<bool, AppError> {
        // TAINT SINK: user_id in DELETE WHERE clause
        let result = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(user_id)  // TAINT: from path/session
            .execute(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    /// Check if email exists.
    pub async fn email_exists(pool: &DbPool, email: &str) -> Result<bool, AppError> {
        // TAINT SINK: email in SELECT WHERE
        let row = sqlx::query("SELECT COUNT(*) as count FROM users WHERE email = ?")
            .bind(email)  // TAINT: from registration form
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let count: i32 = row.get("count");
        Ok(count > 0)
    }
}
