//! Report handlers - Analytics, summaries, and progress tracking.
//!
//! TAINT SOURCE: Date range parameters from query strings.

use actix_web::{web, HttpResponse};

use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::ApiResponse;
use crate::services::{MealService, ReportService, WorkoutService};
use crate::validation::Validators;

/// Date range query parameters
#[derive(Debug, serde::Deserialize)]
pub struct DateRangeQuery {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

/// Single date query parameter
#[derive(Debug, serde::Deserialize)]
pub struct DateQuery {
    pub date: Option<String>,
}

/// GET /api/reports/today - Get today's progress summary
pub async fn get_today_progress(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, AppError> {
    let today = ReportService::today();

    // TAINT FLOW: today (calculated) -> ReportService -> SQL aggregation
    let progress = ReportService::get_daily_progress(&pool, &user_id, &today).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(progress)))
}

/// GET /api/reports/daily - Get daily summary for a specific date
///
/// TAINT SOURCE: web::Query<DateQuery> (date parameter)
pub async fn get_daily_report(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    query: web::Query<DateQuery>,  // TAINT SOURCE
) -> Result<HttpResponse, AppError> {
    let date = query.date.clone().unwrap_or_else(ReportService::today);

    // TAINT TRANSFORM: Validate date format
    Validators::validate_date_format(&date)?;

    // TAINT FLOW: date -> ReportService -> repository -> SQL
    let summary = ReportService::get_daily_summary(&pool, &user_id, &date).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(summary)))
}

/// GET /api/reports/weekly - Get weekly summary
///
/// TAINT SOURCE: web::Query<DateQuery> (week start date)
pub async fn get_weekly_report(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    query: web::Query<DateQuery>,  // TAINT SOURCE
) -> Result<HttpResponse, AppError> {
    // Default to current week start (Monday)
    let week_start = query.date.clone().unwrap_or_else(|| {
        let now = chrono::Utc::now().naive_utc().date();
        let days_since_monday = now.weekday().num_days_from_monday();
        let monday = now - chrono::Duration::days(days_since_monday as i64);
        monday.format("%Y-%m-%d").to_string()
    });

    Validators::validate_date_format(&week_start)?;

    // TAINT FLOW: week_start -> ReportService -> repository -> SQL date range
    let summaries = ReportService::get_weekly_summary(&pool, &user_id, &week_start).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(summaries)))
}

/// GET /api/reports/range - Get summaries for a date range
///
/// TAINT SOURCE: web::Query<DateRangeQuery> (start_date, end_date)
pub async fn get_range_report(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    query: web::Query<DateRangeQuery>,  // TAINT SOURCE
) -> Result<HttpResponse, AppError> {
    let today = ReportService::today();

    // Default to last 7 days
    let end_date = query.end_date.clone().unwrap_or_else(|| today.clone());
    let start_date = query.start_date.clone().unwrap_or_else(|| {
        let end = chrono::NaiveDate::parse_from_str(&end_date, "%Y-%m-%d")
            .unwrap_or_else(|_| chrono::Utc::now().naive_utc().date());
        let start = end - chrono::Duration::days(7);
        start.format("%Y-%m-%d").to_string()
    });

    // TAINT TRANSFORM: Validate dates
    Validators::validate_date_format(&start_date)?;
    Validators::validate_date_format(&end_date)?;

    // TAINT FLOW: date range -> repository -> SQL WHERE BETWEEN
    let summaries = crate::repositories::ScheduleRepository::get_summaries_range(
        &pool, &user_id, &start_date, &end_date,
    ).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(summaries)))
}

/// GET /api/reports/nutrition - Get nutrition averages for date range
///
/// TAINT SOURCE: web::Query<DateRangeQuery>
pub async fn get_nutrition_report(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    query: web::Query<DateRangeQuery>,  // TAINT SOURCE
) -> Result<HttpResponse, AppError> {
    let today = ReportService::today();

    let end_date = query.end_date.clone().unwrap_or_else(|| today.clone());
    let start_date = query.start_date.clone().unwrap_or_else(|| {
        let end = chrono::NaiveDate::parse_from_str(&end_date, "%Y-%m-%d")
            .unwrap_or_else(|_| chrono::Utc::now().naive_utc().date());
        let start = end - chrono::Duration::days(30);
        start.format("%Y-%m-%d").to_string()
    });

    Validators::validate_date_format(&start_date)?;
    Validators::validate_date_format(&end_date)?;

    // TAINT FLOW: date range -> ReportService -> repository -> SQL aggregation
    let nutrition = ReportService::get_nutrition_averages(
        &pool, &user_id, &start_date, &end_date,
    ).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(nutrition)))
}

/// GET /api/reports/progress/{date} - Get progress for a specific date
///
/// TAINT SOURCE: web::Path<String> (date from URL path)
pub async fn get_date_progress(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    path: web::Path<String>,  // TAINT SOURCE
) -> Result<HttpResponse, AppError> {
    let date = path.into_inner();

    Validators::validate_date_format(&date)?;

    // TAINT FLOW: date -> ReportService -> repository -> SQL
    let progress = ReportService::get_daily_progress(&pool, &user_id, &date).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(progress)))
}

/// GET /api/reports/goal-rate - Get goal achievement rate
///
/// TAINT SOURCE: web::Query<DateRangeQuery>
pub async fn get_goal_rate(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    query: web::Query<DateRangeQuery>,  // TAINT SOURCE
) -> Result<HttpResponse, AppError> {
    let today = ReportService::today();

    let end_date = query.end_date.clone().unwrap_or_else(|| today.clone());
    let start_date = query.start_date.clone().unwrap_or_else(|| {
        let end = chrono::NaiveDate::parse_from_str(&end_date, "%Y-%m-%d")
            .unwrap_or_else(|_| chrono::Utc::now().naive_utc().date());
        let start = end - chrono::Duration::days(30);
        start.format("%Y-%m-%d").to_string()
    });

    Validators::validate_date_format(&start_date)?;
    Validators::validate_date_format(&end_date)?;

    // TAINT FLOW: date range -> ReportService -> SQL
    let rate = ReportService::get_goal_achievement_rate(
        &pool, &user_id, &start_date, &end_date,
    ).await?;

    #[derive(serde::Serialize)]
    struct GoalRateResponse {
        achievement_rate: f64,
        start_date: String,
        end_date: String,
    }

    Ok(HttpResponse::Ok().json(ApiResponse::success(GoalRateResponse {
        achievement_rate: rate,
        start_date,
        end_date,
    })))
}

/// GET /api/dashboard - Get dashboard data (combined endpoint)
pub async fn get_dashboard(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, AppError> {
    let today = ReportService::today();

    // Get today's progress
    let progress = ReportService::get_daily_progress(&pool, &user_id, &today).await?;

    // Get today's meals
    let meals = MealService::get_meals_for_date(&pool, &user_id, &today).await?;

    // Get today's workouts
    let workouts = WorkoutService::get_workouts_for_date(&pool, &user_id, &today).await?;

    #[derive(serde::Serialize)]
    struct DashboardResponse {
        date: String,
        progress: crate::models::GoalProgress,
        meals: Vec<crate::models::MealWithTotals>,
        workouts: Vec<crate::models::WorkoutWithStats>,
    }

    Ok(HttpResponse::Ok().json(ApiResponse::success(DashboardResponse {
        date: today,
        progress,
        meals,
        workouts,
    })))
}
