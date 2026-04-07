//! Schedule handlers - CRUD operations for daily schedules and goals.
//!
//! TAINT SOURCE: All HTTP input flows through this module.

use actix_web::{web, HttpResponse};

use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::{
    ApiResponse, CreateScheduleRequest, ScheduleWithDay, UpdateScheduleRequest, WeeklySchedule,
};
use crate::repositories::ScheduleRepository;
use crate::validation::Validators;

/// POST /api/schedules - Create or update a schedule for a day
///
/// TAINT SOURCE: web::Json<CreateScheduleRequest>
pub async fn upsert_schedule(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    body: web::Json<CreateScheduleRequest>,  // TAINT SOURCE
) -> Result<HttpResponse, AppError> {
    let request = body.into_inner();

    // TAINT TRANSFORM: Validate
    Validators::validate_schedule(&request)?;

    // TAINT FLOW: request -> ScheduleRepository -> SQL UPSERT
    let schedule = ScheduleRepository::upsert_schedule(&pool, &user_id, &request).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(ScheduleWithDay::from(schedule))))
}

/// GET /api/schedules/{day} - Get schedule for a specific day
///
/// TAINT SOURCE: web::Path<i32> (day of week)
pub async fn get_schedule(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    path: web::Path<i32>,  // TAINT SOURCE: day_of_week from URL
) -> Result<HttpResponse, AppError> {
    let day_of_week = path.into_inner();

    // Validate day of week
    if !(0..=6).contains(&day_of_week) {
        return Err(AppError::Validation("Day of week must be 0-6".to_string()));
    }

    // TAINT FLOW: day_of_week -> repository -> SQL WHERE
    let schedule = ScheduleRepository::find_by_day(&pool, &user_id, day_of_week)
        .await?
        .ok_or(AppError::NotFound("Schedule not found for this day".to_string()))?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(ScheduleWithDay::from(schedule))))
}

/// GET /api/schedules - Get all schedules (weekly view)
pub async fn get_weekly_schedule(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, AppError> {
    // TAINT FLOW: user_id -> repository -> SQL SELECT
    let schedules = ScheduleRepository::get_weekly_schedule(&pool, &user_id).await?;

    let schedules_with_day: Vec<ScheduleWithDay> = schedules
        .into_iter()
        .map(ScheduleWithDay::from)
        .collect();

    let total_calories: i32 = schedules_with_day
        .iter()
        .map(|s| s.schedule.target_calories)
        .sum();

    let avg_calories = if schedules_with_day.is_empty() {
        0.0
    } else {
        total_calories as f64 / schedules_with_day.len() as f64
    };

    let weekly = WeeklySchedule {
        schedules: schedules_with_day,
        total_weekly_calories: total_calories,
        avg_daily_calories: avg_calories,
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(weekly)))
}

/// PUT /api/schedules/{day} - Update a schedule
///
/// TAINT SOURCE: Both Path (day) and Json body
pub async fn update_schedule(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    path: web::Path<i32>,  // TAINT SOURCE
    body: web::Json<UpdateScheduleRequest>,  // TAINT SOURCE
) -> Result<HttpResponse, AppError> {
    let day_of_week = path.into_inner();
    let request = body.into_inner();

    // Validate day of week
    if !(0..=6).contains(&day_of_week) {
        return Err(AppError::Validation("Day of week must be 0-6".to_string()));
    }

    // TAINT FLOW: request -> repository -> SQL UPDATE
    let schedule = ScheduleRepository::update_schedule(&pool, &user_id, day_of_week, &request).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(ScheduleWithDay::from(schedule))))
}

/// DELETE /api/schedules/{day} - Delete a schedule
///
/// TAINT SOURCE: web::Path<i32>
pub async fn delete_schedule(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    path: web::Path<i32>,  // TAINT SOURCE
) -> Result<HttpResponse, AppError> {
    let day_of_week = path.into_inner();

    // TAINT FLOW: day_of_week -> repository -> SQL DELETE
    let deleted = ScheduleRepository::delete_schedule(&pool, &user_id, day_of_week).await?;

    if deleted {
        Ok(HttpResponse::Ok().json(ApiResponse::<()>::success_with_message((), "Schedule deleted")))
    } else {
        Err(AppError::NotFound("Schedule not found".to_string()))
    }
}

/// POST /api/schedules/copy-week - Copy current week schedule to all days
pub async fn copy_schedule_to_week(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    body: web::Json<CreateScheduleRequest>,  // TAINT SOURCE: Template schedule
) -> Result<HttpResponse, AppError> {
    let template = body.into_inner();

    // TAINT TRANSFORM: Validate template
    Validators::validate_schedule(&template)?;

    // Create schedule for each day of the week
    let mut created = Vec::new();
    for day in 0..=6 {
        let day_request = CreateScheduleRequest {
            day_of_week: day,
            target_calories: template.target_calories,
            target_protein_grams: template.target_protein_grams,
            target_carbs_grams: template.target_carbs_grams,
            target_fat_grams: template.target_fat_grams,
            planned_workouts: template.planned_workouts.clone(),
            notes: template.notes.clone(),
        };

        // TAINT FLOW: day_request -> repository -> SQL INSERT
        let schedule = ScheduleRepository::upsert_schedule(&pool, &user_id, &day_request).await?;
        created.push(ScheduleWithDay::from(schedule));
    }

    Ok(HttpResponse::Created().json(ApiResponse::success(created)))
}
