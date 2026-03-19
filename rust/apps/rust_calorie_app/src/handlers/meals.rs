//! Meal handlers - CRUD operations for meals and food items.
//!
//! TAINT SOURCE: All HTTP input (Json, Path, Query) are taint sources.
//! Multi-hop cross-file flow demonstrated here:
//! 1. This file: HTTP input received (SOURCE)
//! 2. services/meal_service.rs: Business logic (TRANSFORM)
//! 3. repositories/meal_repo.rs: SQL execution (SINK)

use actix_web::{web, HttpResponse};

use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::{
    ApiResponse, CreateFoodItemRequest, CreateMealRequest, MealSearchQuery, UpdateMealRequest,
};
use crate::models::food_item::{FoodItemSearchQuery, UpdateFoodItemRequest};
use crate::services::MealService;
use crate::validation::Validators;

// ==================== Food Items ====================

/// POST /api/food-items - Create a food item
///
/// TAINT SOURCE: web::Json<CreateFoodItemRequest>
pub async fn create_food_item(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    body: web::Json<CreateFoodItemRequest>,  // TAINT SOURCE
) -> Result<HttpResponse, AppError> {
    let request = body.into_inner();  // TAINT: extracted from HTTP

    // TAINT FLOW: request -> MealService -> MealRepository -> SQL INSERT
    let food_item = MealService::create_food_item(&pool, &user_id, request).await?;

    Ok(HttpResponse::Created().json(ApiResponse::success(food_item)))
}

/// GET /api/food-items/{id} - Get a food item
///
/// TAINT SOURCE: web::Path<String> (URL path parameter)
pub async fn get_food_item(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    path: web::Path<String>,  // TAINT SOURCE: Path parameter
) -> Result<HttpResponse, AppError> {
    let food_item_id = path.into_inner();  // TAINT: from URL

    // TAINT FLOW: food_item_id -> MealService -> MealRepository -> SQL SELECT
    let food_item = MealService::get_food_item(&pool, &user_id, &food_item_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(food_item)))
}

/// GET /api/food-items - List food items with optional search
///
/// TAINT SOURCE: web::Query<FoodItemSearchQuery>
pub async fn list_food_items(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    query: web::Query<FoodItemSearchQuery>,  // TAINT SOURCE: Query parameters
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();  // TAINT: from URL query string

    // TAINT FLOW: params -> MealService -> MealRepository -> SQL with LIKE
    let food_items = MealService::list_food_items(
        &pool,
        &user_id,
        params.search,      // TAINT: search term -> SQL LIKE
        params.category,    // TAINT: category -> SQL WHERE
        params.page.unwrap_or(0),
        params.per_page.unwrap_or(20),
    )
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(food_items)))
}

// ==================== Meals ====================

/// POST /api/meals - Create a meal entry
///
/// TAINT SOURCE: web::Json<CreateMealRequest>
/// This is a primary entry point demonstrating deep multi-hop taint flow:
/// HTTP body -> validation -> nutrition calculation -> SQL INSERT
pub async fn create_meal(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    body: web::Json<CreateMealRequest>,  // TAINT SOURCE: HTTP request body
) -> Result<HttpResponse, AppError> {
    let request = body.into_inner();  // TAINT: All fields from HTTP

    // TAINT TRANSFORM: Validate input
    Validators::validate_meal(&request)?;

    // TAINT FLOW: request -> MealService (calculate) -> MealRepository -> SQL
    let meal = MealService::create_meal(&pool, &user_id, request).await?;

    Ok(HttpResponse::Created().json(ApiResponse::success(meal)))
}

/// GET /api/meals/{id} - Get a meal by ID
///
/// TAINT SOURCE: web::Path<String>
pub async fn get_meal(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    path: web::Path<String>,  // TAINT SOURCE
) -> Result<HttpResponse, AppError> {
    let meal_id = path.into_inner();  // TAINT: from URL path

    // TAINT FLOW: meal_id -> service -> repository -> SQL WHERE
    let meal = MealService::get_meal(&pool, &user_id, &meal_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(meal)))
}

/// GET /api/meals - Search meals with filters
///
/// TAINT SOURCE: web::Query<MealSearchQuery> - all query parameters
pub async fn search_meals(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    query: web::Query<MealSearchQuery>,  // TAINT SOURCE: All query params
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();

    // TAINT FLOW: params -> MealService -> MealRepository -> dynamic SQL query
    // This demonstrates taint flowing through multiple WHERE clauses
    let meals = MealService::search_meals(&pool, &user_id, params).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(meals)))
}

/// PUT /api/meals/{id} - Update a meal
///
/// TAINT SOURCE: Both Path (meal_id) and Json (update fields)
pub async fn update_meal(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    path: web::Path<String>,  // TAINT SOURCE
    body: web::Json<UpdateMealRequest>,  // TAINT SOURCE
) -> Result<HttpResponse, AppError> {
    let meal_id = path.into_inner();  // TAINT
    let request = body.into_inner();   // TAINT

    // TAINT FLOW: meal_id + request -> service -> repository -> SQL UPDATE
    let meal = MealService::update_meal(&pool, &user_id, &meal_id, request).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(meal)))
}

/// DELETE /api/meals/{id} - Delete a meal
///
/// TAINT SOURCE: web::Path<String>
pub async fn delete_meal(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    path: web::Path<String>,  // TAINT SOURCE
) -> Result<HttpResponse, AppError> {
    let meal_id = path.into_inner();  // TAINT

    // TAINT FLOW: meal_id -> service -> repository -> SQL DELETE
    let deleted = MealService::delete_meal(&pool, &user_id, &meal_id).await?;

    if deleted {
        Ok(HttpResponse::Ok().json(ApiResponse::<()>::success_with_message((), "Meal deleted")))
    } else {
        Err(AppError::NotFound("Meal not found".to_string()))
    }
}

/// GET /api/meals/today - Get today's meals
pub async fn get_todays_meals(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, AppError> {
    let today = crate::services::ReportService::today();

    let meals = MealService::get_meals_for_date(&pool, &user_id, &today).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(meals)))
}

/// GET /api/meals/date/{date} - Get meals for a specific date
///
/// TAINT SOURCE: web::Path<String> (date parameter)
pub async fn get_meals_by_date(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<String>,
    path: web::Path<String>,  // TAINT SOURCE: date from URL
) -> Result<HttpResponse, AppError> {
    let date = path.into_inner();  // TAINT

    // Validate date format
    Validators::validate_date_format(&date)?;

    // TAINT FLOW: date -> service -> repository -> SQL WHERE
    let meals = MealService::get_meals_for_date(&pool, &user_id, &date).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(meals)))
}
