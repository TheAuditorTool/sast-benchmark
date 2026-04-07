//! Workout service - Business logic for workout and exercise operations.
//!
//! TAINT TRANSFORM: HTTP input -> validation -> calorie calculation -> repository

use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::{
    CreateExerciseTypeRequest, CreateWorkoutRequest, ExerciseType, Intensity,
    UpdateWorkoutRequest, Workout, WorkoutSearchQuery, WorkoutWithStats,
};
use crate::repositories::WorkoutRepository;
use crate::services::CalorieCalculator;
use crate::validation::Validators;

/// Service for workout business logic
pub struct WorkoutService;

impl WorkoutService {
    // ==================== Exercise Types ====================

    /// Create an exercise type with validation.
    ///
    /// TAINT TRANSFORM: HTTP request -> validate -> repository
    pub async fn create_exercise_type(
        pool: &DbPool,
        user_id: &str,
        request: CreateExerciseTypeRequest,
    ) -> Result<ExerciseType, AppError> {
        // TAINT TRANSFORM: Validate HTTP input
        Validators::validate_exercise_type(&request)?;

        // Normalize category
        let mut normalized = request;
        if let Some(ref mut cat) = normalized.category {
            *cat = cat.to_lowercase();
        }

        // TAINT FLOW: validated request -> WorkoutRepository -> SQL INSERT
        WorkoutRepository::create_exercise_type(pool, user_id, &normalized).await
    }

    /// Get exercise type by ID with ownership check.
    pub async fn get_exercise_type(
        pool: &DbPool,
        user_id: &str,
        exercise_type_id: &str,
    ) -> Result<ExerciseType, AppError> {
        let exercise = WorkoutRepository::find_exercise_type_by_id(pool, exercise_type_id)
            .await?
            .ok_or(AppError::NotFound("Exercise type not found".to_string()))?;

        if exercise.user_id != user_id {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }

        Ok(exercise)
    }

    /// List exercise types by category.
    pub async fn list_exercise_types(
        pool: &DbPool,
        user_id: &str,
        category: Option<String>,
    ) -> Result<Vec<ExerciseType>, AppError> {
        // TAINT FLOW: category from query -> repository -> SQL WHERE
        WorkoutRepository::list_exercise_types(pool, user_id, category.as_deref()).await
    }

    // ==================== Workouts ====================

    /// Create a workout entry with calorie calculation.
    ///
    /// TAINT TRANSFORM: HTTP request -> validate -> calculate calories -> repository
    /// Multi-hop cross-file flow:
    /// 1. handlers/workouts.rs receives CreateWorkoutRequest (SOURCE)
    /// 2. Here: validate and calculate calories (TRANSFORM)
    /// 3. WorkoutRepository::create_workout executes SQL (SINK)
    pub async fn create_workout(
        pool: &DbPool,
        user_id: &str,
        request: CreateWorkoutRequest,
    ) -> Result<WorkoutWithStats, AppError> {
        // TAINT TRANSFORM: Validate HTTP input
        Validators::validate_workout(&request)?;

        // Validate intensity if provided
        let intensity = request
            .intensity
            .as_ref()
            .and_then(|i| Intensity::from_str(i))
            .unwrap_or(Intensity::Moderate);

        // Calculate calories burned
        let calculated_calories = if let Some(ref exercise_type_id) = request.exercise_type_id {
            // TAINT FLOW: exercise_type_id -> repository query
            let exercise_type = WorkoutRepository::find_exercise_type_by_id(pool, exercise_type_id)
                .await?
                .ok_or(AppError::NotFound("Exercise type not found".to_string()))?;

            // Verify ownership
            if exercise_type.user_id != user_id {
                return Err(AppError::Forbidden("Access to exercise type denied".to_string()));
            }

            // TAINT TRANSFORM: Calculate calories using HTTP input (duration) and exercise data
            Some(CalorieCalculator::calculate_workout_calories(
                request.duration_minutes,
                exercise_type.calories_per_minute,
                &intensity,
            ))
        } else if let Some(calories) = request.calories_burned {
            // Use provided calories
            Some(calories)
        } else {
            // Estimate using MET values
            Some(CalorieCalculator::estimate_calories_from_duration(
                request.duration_minutes,
                &intensity,
            ))
        };

        // TAINT FLOW: request + calculated -> WorkoutRepository -> SQL INSERT
        let workout =
            WorkoutRepository::create_workout(pool, user_id, &request, calculated_calories).await?;

        Ok(WorkoutWithStats::from(workout))
    }

    /// Get workout by ID with ownership check.
    pub async fn get_workout(
        pool: &DbPool,
        user_id: &str,
        workout_id: &str,
    ) -> Result<WorkoutWithStats, AppError> {
        // TAINT FLOW: workout_id -> repository query
        let workout = WorkoutRepository::find_workout_by_id(pool, workout_id)
            .await?
            .ok_or(AppError::NotFound("Workout not found".to_string()))?;

        if workout.user_id != user_id {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }

        Ok(WorkoutWithStats::from(workout))
    }

    /// Search workouts with filters.
    ///
    /// TAINT TRANSFORM: query params -> validate -> repository
    pub async fn search_workouts(
        pool: &DbPool,
        user_id: &str,
        query: WorkoutSearchQuery,
    ) -> Result<Vec<WorkoutWithStats>, AppError> {
        // TAINT TRANSFORM: Validate query params
        let mut validated_query = query;

        // Validate intensity if provided
        if let Some(ref i) = validated_query.intensity {
            if Intensity::from_str(i).is_none() {
                return Err(AppError::Validation("Invalid intensity".to_string()));
            }
        }

        // Validate date format if provided
        if let Some(ref date) = validated_query.start_date {
            Validators::validate_date_format(date)?;
        }
        if let Some(ref date) = validated_query.end_date {
            Validators::validate_date_format(date)?;
        }

        // TAINT FLOW: validated query -> WorkoutRepository -> dynamic SQL
        let workouts = WorkoutRepository::search_workouts(pool, user_id, &validated_query).await?;

        Ok(workouts.into_iter().map(WorkoutWithStats::from).collect())
    }

    /// Update a workout.
    ///
    /// TAINT TRANSFORM: HTTP request -> validate -> repository
    pub async fn update_workout(
        pool: &DbPool,
        user_id: &str,
        workout_id: &str,
        request: UpdateWorkoutRequest,
    ) -> Result<WorkoutWithStats, AppError> {
        // Verify ownership first
        let existing = WorkoutRepository::find_workout_by_id(pool, workout_id)
            .await?
            .ok_or(AppError::NotFound("Workout not found".to_string()))?;

        if existing.user_id != user_id {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }

        // Validate intensity if provided
        if let Some(ref i) = request.intensity {
            if Intensity::from_str(i).is_none() {
                return Err(AppError::Validation("Invalid intensity".to_string()));
            }
        }

        // TAINT FLOW: request -> WorkoutRepository -> SQL UPDATE
        let workout = WorkoutRepository::update_workout(pool, workout_id, user_id, &request).await?;

        Ok(WorkoutWithStats::from(workout))
    }

    /// Delete a workout.
    pub async fn delete_workout(
        pool: &DbPool,
        user_id: &str,
        workout_id: &str,
    ) -> Result<bool, AppError> {
        // TAINT FLOW: workout_id + user_id -> repository -> SQL DELETE
        WorkoutRepository::delete_workout(pool, workout_id, user_id).await
    }

    /// Get daily workout totals.
    pub async fn get_daily_totals(
        pool: &DbPool,
        user_id: &str,
        date: &str,
    ) -> Result<(i32, i32, i32), AppError> {
        // TAINT FLOW: date -> WorkoutRepository -> SQL aggregation
        WorkoutRepository::get_daily_totals(pool, user_id, date).await
    }

    /// Get workouts for a specific date.
    pub async fn get_workouts_for_date(
        pool: &DbPool,
        user_id: &str,
        date: &str,
    ) -> Result<Vec<WorkoutWithStats>, AppError> {
        let query = WorkoutSearchQuery {
            category: None,
            intensity: None,
            start_date: Some(format!("{}T00:00:00", date)),
            end_date: Some(format!("{}T23:59:59", date)),
            min_duration: None,
            max_duration: None,
            search: None,
            page: Some(0),
            per_page: Some(100),
        };

        Self::search_workouts(pool, user_id, query).await
    }
}
