//! Workout repository - SQL queries for workout and exercise management.
//!
//! TAINT SINK: All methods receive data from upstream services/handlers.
//! Cross-file flow: handlers/workouts.rs -> services/workout_service.rs -> here

use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::{
    CreateExerciseTypeRequest, CreateWorkoutRequest, ExerciseType, UpdateWorkoutRequest,
    Workout, WorkoutSearchQuery,
};
use sqlx::Row;
use uuid::Uuid;

/// Repository for workout database operations
pub struct WorkoutRepository;

impl WorkoutRepository {
    // ==================== Exercise Types ====================

    /// Create an exercise type.
    ///
    /// TAINT SINK: request fields flow to INSERT
    pub async fn create_exercise_type(
        pool: &DbPool,
        user_id: &str,
        request: &CreateExerciseTypeRequest,
    ) -> Result<ExerciseType, AppError> {
        let id = Uuid::new_v4().to_string();

        // TAINT SINK: sqlx::query with exercise type data
        sqlx::query(
            r#"
            INSERT INTO exercise_types (
                id, user_id, name, calories_per_minute, category, met_value, created_at
            )
            VALUES (?, ?, ?, ?, ?, ?, datetime('now'))
            "#,
        )
        .bind(&id)
        .bind(user_id)
        .bind(&request.name)                          // TAINT: from HTTP
        .bind(request.calories_per_minute)            // TAINT: from HTTP
        .bind(request.category.as_deref().unwrap_or("cardio"))  // TAINT
        .bind(request.met_value.unwrap_or(5.0))       // TAINT: from HTTP
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Self::find_exercise_type_by_id(pool, &id)
            .await?
            .ok_or(AppError::NotFound("Exercise type not found".to_string()))
    }

    /// Find exercise type by ID.
    pub async fn find_exercise_type_by_id(
        pool: &DbPool,
        id: &str,
    ) -> Result<Option<ExerciseType>, AppError> {
        let exercise = sqlx::query_as::<_, ExerciseType>(
            "SELECT * FROM exercise_types WHERE id = ?",
        )
        .bind(id)  // TAINT: from path param
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(exercise)
    }

    /// List user's exercise types.
    pub async fn list_exercise_types(
        pool: &DbPool,
        user_id: &str,
        category: Option<&str>,
    ) -> Result<Vec<ExerciseType>, AppError> {
        let query = if let Some(cat) = category {
            // TAINT SINK: category in WHERE clause
            sqlx::query_as::<_, ExerciseType>(
                "SELECT * FROM exercise_types WHERE user_id = ? AND category = ? ORDER BY name",
            )
            .bind(user_id)
            .bind(cat)  // TAINT: from query param
            .fetch_all(pool)
            .await
        } else {
            sqlx::query_as::<_, ExerciseType>(
                "SELECT * FROM exercise_types WHERE user_id = ? ORDER BY name",
            )
            .bind(user_id)
            .fetch_all(pool)
            .await
        };

        query.map_err(|e| AppError::Database(e.to_string()))
    }

    // ==================== Workouts ====================

    // vuln-code-snippet start sqliCalorieCreateWorkout
    /// Create a workout entry.
    ///
    /// TAINT SINK: All CreateWorkoutRequest fields flow to INSERT
    /// End of multi-hop flow: handlers -> service (calculate calories) -> here
    pub async fn create_workout(
        pool: &DbPool,
        user_id: &str,
        request: &CreateWorkoutRequest,
        calculated_calories: Option<i32>,
    ) -> Result<Workout, AppError> {
        let id = Uuid::new_v4().to_string();
        let calories = calculated_calories.or(request.calories_burned).unwrap_or(0);
        let intensity = request.intensity.as_deref().unwrap_or("moderate");

        // TAINT SINK: All values from HTTP request
        sqlx::query(
            r#"
            INSERT INTO workouts (
                id, user_id, exercise_type_id, name, duration_minutes,
                calories_burned, intensity, performed_at, notes,
                heart_rate_avg, created_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'))
            "#,
        )
        // vuln-code-snippet target-line sqliCalorieCreateWorkout
        .bind(&id)
        .bind(user_id)
        .bind(&request.exercise_type_id)          // TAINT: optional FK from HTTP
        .bind(&request.name)                       // TAINT: from HTTP
        .bind(request.duration_minutes)           // TAINT: from HTTP
        .bind(calories)                            // TAINT: calculated from HTTP
        .bind(intensity)                           // TAINT: from HTTP
        .bind(&request.performed_at)               // TAINT: from HTTP
        .bind(&request.notes)                      // TAINT: from HTTP
        .bind(request.heart_rate_avg)             // TAINT: from HTTP
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Self::find_workout_by_id(pool, &id)
            .await?
            .ok_or(AppError::NotFound("Workout not found".to_string()))
    }
    // vuln-code-snippet end sqliCalorieCreateWorkout

    /// Find workout by ID.
    pub async fn find_workout_by_id(pool: &DbPool, id: &str) -> Result<Option<Workout>, AppError> {
        let workout = sqlx::query_as::<_, Workout>("SELECT * FROM workouts WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(workout)
    }

    // vuln-code-snippet start sqliCalorieSearchWorkouts
    /// Search workouts with filters.
    ///
    /// TAINT SINK: All WorkoutSearchQuery fields flow to dynamic SQL
    pub async fn search_workouts(
        pool: &DbPool,
        user_id: &str,
        params: &WorkoutSearchQuery,
    ) -> Result<Vec<Workout>, AppError> {
        let mut query = String::from("SELECT * FROM workouts WHERE user_id = ?");

        // Using a simpler approach for bindings
        // In production, use a query builder like sea-query

        // TAINT: intensity filter
        if let Some(ref intensity) = params.intensity {
            // vuln-code-snippet target-line sqliCalorieSearchWorkouts
            query.push_str(&format!(" AND intensity = '{}'", intensity.replace("'", "''")));
        }

        // TAINT: date range
        if let Some(ref start) = params.start_date {
            query.push_str(&format!(" AND performed_at >= '{}'", start.replace("'", "''")));
        }
        if let Some(ref end) = params.end_date {
            query.push_str(&format!(" AND performed_at <= '{}'", end.replace("'", "''")));
        }

        // TAINT: duration range
        if let Some(min_dur) = params.min_duration {
            query.push_str(&format!(" AND duration_minutes >= {}", min_dur));
        }
        if let Some(max_dur) = params.max_duration {
            query.push_str(&format!(" AND duration_minutes <= {}", max_dur));
        }

        // TAINT: search in name - VULNERABLE pattern for demonstration
        if let Some(ref search) = params.search {
            //String interpolation (for taint analysis demonstration)
            query.push_str(&format!(" AND name LIKE '%{}%'", search.replace("'", "''")));
        }

        query.push_str(" ORDER BY performed_at DESC");

        // Pagination
        let limit = params.per_page.unwrap_or(20).min(100);
        let offset = params.page.unwrap_or(0) * limit;
        query.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

        // TAINT SINK: Execute dynamically built query
        let workouts = sqlx::query_as::<_, Workout>(&query)
            .bind(user_id)
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(workouts)
    }
    // vuln-code-snippet end sqliCalorieSearchWorkouts

    // vuln-code-snippet start sqliCalorieUpdateWorkout
    /// Update a workout.
    ///
    /// TAINT SINK: UpdateWorkoutRequest fields flow to UPDATE
    pub async fn update_workout(
        pool: &DbPool,
        workout_id: &str,
        user_id: &str,
        request: &UpdateWorkoutRequest,
    ) -> Result<Workout, AppError> {
        let mut updates = Vec::new();

        // Build SET clause dynamically - TAINT flows through all fields
        if let Some(ref name) = request.name {
            // vuln-code-snippet target-line sqliCalorieUpdateWorkout
            updates.push(format!("name = '{}'", name.replace("'", "''")));
        }
        if let Some(duration) = request.duration_minutes {
            updates.push(format!("duration_minutes = {}", duration));
        }
        if let Some(calories) = request.calories_burned {
            updates.push(format!("calories_burned = {}", calories));
        }
        if let Some(ref intensity) = request.intensity {
            updates.push(format!("intensity = '{}'", intensity.replace("'", "''")));
        }
        if let Some(ref performed_at) = request.performed_at {
            updates.push(format!("performed_at = '{}'", performed_at.replace("'", "''")));
        }
        if let Some(ref notes) = request.notes {
            updates.push(format!("notes = '{}'", notes.replace("'", "''")));
        }
        if let Some(hr) = request.heart_rate_avg {
            updates.push(format!("heart_rate_avg = {}", hr));
        }

        if updates.is_empty() {
            return Self::find_workout_by_id(pool, workout_id)
                .await?
                .ok_or(AppError::NotFound("Workout not found".to_string()));
        }

        // TAINT SINK: Dynamic UPDATE query
        let query = format!(
            "UPDATE workouts SET {} WHERE id = ? AND user_id = ?",
            updates.join(", ")
        );

        let result = sqlx::query(&query)
            .bind(workout_id)
            .bind(user_id)
            .execute(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Workout not found or access denied".to_string()));
        }

        Self::find_workout_by_id(pool, workout_id)
            .await?
            .ok_or(AppError::NotFound("Workout not found".to_string()))
    }
    // vuln-code-snippet end sqliCalorieUpdateWorkout

    // vuln-code-snippet start sqliCalorieDeleteWorkout
    /// Delete a workout.
    pub async fn delete_workout(
        pool: &DbPool,
        workout_id: &str,
        user_id: &str,
    ) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM workouts WHERE id = ? AND user_id = ?")
            // vuln-code-snippet target-line sqliCalorieDeleteWorkout
            .bind(workout_id)
            .bind(user_id)
            .execute(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }
    // vuln-code-snippet end sqliCalorieDeleteWorkout

    /// Get daily workout totals.
    ///
    /// TAINT SINK: date param in WHERE
    pub async fn get_daily_totals(
        pool: &DbPool,
        user_id: &str,
        date: &str,
    ) -> Result<(i32, i32, i32), AppError> {
        let row = sqlx::query(
            r#"
            SELECT
                COALESCE(SUM(calories_burned), 0) as total_calories,
                COALESCE(SUM(duration_minutes), 0) as total_duration,
                COUNT(*) as workout_count
            FROM workouts
            WHERE user_id = ? AND date(performed_at) = date(?)
            "#,
        )
        .bind(user_id)
        .bind(date)  // TAINT: from query/calculated
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok((
            row.get::<i32, _>("total_calories"),
            row.get::<i32, _>("total_duration"),
            row.get::<i32, _>("workout_count"),
        ))
    }
}
