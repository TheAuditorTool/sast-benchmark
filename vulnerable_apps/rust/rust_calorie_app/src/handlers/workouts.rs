//! Workout handlers - CRUD operations for workouts and exercise types.
//!
//! TAINT SOURCE: All HTTP input flows through this module.

use actix_web::{web, HttpResponse};

use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::{
    ApiResponse, CreateExerciseTypeRequest, CreateWorkoutRequest, UpdateWorkoutRequest,
    WorkoutSearchQuery,
};
use crate::services::WorkoutService;
use crate::validation::Validators;

// ==================== Exercise Types ====================

/// POST /api/exercise-types - Create an exercise type
///
/// TAINT SOURCE: web::Json<CreateExerciseTypeRequest>
pub async fn create_exercise_type(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    body: web::Json<CreateExerciseTypeRequest>,  // TAINT SOURCE
) -> Result<HttpResponse, AppError> {
    let request = body.into_inner();

    // TAINT FLOW: request -> WorkoutService -> WorkoutRepository -> SQL
    let exercise_type = WorkoutService::create_exercise_type(&pool, &user_id, request).await?;

    Ok(HttpResponse::Created().json(ApiResponse::success(exercise_type)))
}

/// GET /api/exercise-types/{id} - Get an exercise type
pub async fn get_exercise_type(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    path: web::Path<String>,  // TAINT SOURCE
) -> Result<HttpResponse, AppError> {
    let exercise_type_id = path.into_inner();

    let exercise_type = WorkoutService::get_exercise_type(&pool, &user_id, &exercise_type_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(exercise_type)))
}

/// GET /api/exercise-types - List exercise types
///
/// TAINT SOURCE: web::Query with category filter
pub async fn list_exercise_types(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    query: web::Query<CategoryFilter>,  // TAINT SOURCE
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();

    // TAINT FLOW: category -> WorkoutService -> repository -> SQL WHERE
    let exercise_types = WorkoutService::list_exercise_types(&pool, &user_id, params.category).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(exercise_types)))
}

/// Category filter for query params
#[derive(Debug, serde::Deserialize)]
pub struct CategoryFilter {
    pub category: Option<String>,
}

// ==================== Workouts ====================

/// POST /api/workouts - Create a workout entry
///
/// TAINT SOURCE: web::Json<CreateWorkoutRequest>
/// Multi-hop: HTTP body -> validation -> calorie calculation -> SQL
pub async fn create_workout(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    body: web::Json<CreateWorkoutRequest>,  // TAINT SOURCE
) -> Result<HttpResponse, AppError> {
    let request = body.into_inner();

    // TAINT TRANSFORM: Validate
    Validators::validate_workout(&request)?;

    // TAINT FLOW: request -> WorkoutService (calculate) -> WorkoutRepository -> SQL
    let workout = WorkoutService::create_workout(&pool, &user_id, request).await?;

    Ok(HttpResponse::Created().json(ApiResponse::success(workout)))
}

/// GET /api/workouts/{id} - Get a workout by ID
///
/// TAINT SOURCE: web::Path<String>
pub async fn get_workout(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    path: web::Path<String>,  // TAINT SOURCE
) -> Result<HttpResponse, AppError> {
    let workout_id = path.into_inner();

    let workout = WorkoutService::get_workout(&pool, &user_id, &workout_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(workout)))
}

/// GET /api/workouts - Search workouts with filters
///
/// TAINT SOURCE: web::Query<WorkoutSearchQuery>
pub async fn search_workouts(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    query: web::Query<WorkoutSearchQuery>,  // TAINT SOURCE
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();

    // TAINT FLOW: params -> service -> repository -> dynamic SQL
    let workouts = WorkoutService::search_workouts(&pool, &user_id, params).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(workouts)))
}

/// PUT /api/workouts/{id} - Update a workout
///
/// TAINT SOURCE: Both Path and Json body
pub async fn update_workout(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    path: web::Path<String>,  // TAINT SOURCE
    body: web::Json<UpdateWorkoutRequest>,  // TAINT SOURCE
) -> Result<HttpResponse, AppError> {
    let workout_id = path.into_inner();
    let request = body.into_inner();

    let workout = WorkoutService::update_workout(&pool, &user_id, &workout_id, request).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(workout)))
}

/// DELETE /api/workouts/{id} - Delete a workout
pub async fn delete_workout(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    path: web::Path<String>,  // TAINT SOURCE
) -> Result<HttpResponse, AppError> {
    let workout_id = path.into_inner();

    let deleted = WorkoutService::delete_workout(&pool, &user_id, &workout_id).await?;

    if deleted {
        Ok(HttpResponse::Ok().json(ApiResponse::<()>::success_with_message((), "Workout deleted")))
    } else {
        Err(AppError::NotFound("Workout not found".to_string()))
    }
}

/// GET /api/workouts/today - Get today's workouts
pub async fn get_todays_workouts(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, AppError> {
    let today = crate::services::ReportService::today();

    let workouts = WorkoutService::get_workouts_for_date(&pool, &user_id, &today).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(workouts)))
}

/// GET /api/workouts/date/{date} - Get workouts for a specific date
///
/// TAINT SOURCE: web::Path<String> (date)
pub async fn get_workouts_by_date(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    path: web::Path<String>,  // TAINT SOURCE
) -> Result<HttpResponse, AppError> {
    let date = path.into_inner();

    Validators::validate_date_format(&date)?;

    let workouts = WorkoutService::get_workouts_for_date(&pool, &user_id, &date).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(workouts)))
}
