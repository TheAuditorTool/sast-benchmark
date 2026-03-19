//! Meal service - Business logic for meal and food item operations.
//!
//! TAINT TRANSFORM: HTTP input -> validation -> enrichment -> repository
//! This is the middle layer of the multi-hop taint flow.

use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::{
    CreateFoodItemRequest, CreateMealRequest, FoodItem, Meal, MealSearchQuery,
    MealType, MealWithTotals, NutritionInfo, UpdateMealRequest,
};
use crate::repositories::MealRepository;
use crate::validation::Validators;

/// Service for meal business logic
pub struct MealService;

impl MealService {
    // ==================== Food Items ====================

    /// Create a food item with validation.
    ///
    /// TAINT TRANSFORM: HTTP request -> validate -> repository
    pub async fn create_food_item(
        pool: &DbPool,
        user_id: &str,
        request: CreateFoodItemRequest,
    ) -> Result<FoodItem, AppError> {
        // TAINT TRANSFORM: Validate HTTP input
        Validators::validate_food_item(&request)?;

        // Normalize category
        let mut normalized_request = request;
        if let Some(ref mut cat) = normalized_request.category {
            *cat = cat.to_lowercase();
        }

        // TAINT FLOW: validated request -> MealRepository -> SQL
        MealRepository::create_food_item(pool, user_id, &normalized_request).await
    }

    /// Get food item by ID.
    pub async fn get_food_item(
        pool: &DbPool,
        user_id: &str,
        food_item_id: &str,
    ) -> Result<FoodItem, AppError> {
        let item = MealRepository::find_food_item_by_id(pool, food_item_id)
            .await?
            .ok_or(AppError::NotFound("Food item not found".to_string()))?;

        // Verify ownership
        if item.user_id != user_id {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }

        Ok(item)
    }

    /// List food items with search.
    ///
    /// TAINT TRANSFORM: query params -> sanitize -> repository
    pub async fn list_food_items(
        pool: &DbPool,
        user_id: &str,
        search: Option<String>,
        category: Option<String>,
        page: u32,
        per_page: u32,
    ) -> Result<Vec<FoodItem>, AppError> {
        let limit = per_page.min(100);
        let offset = page * limit;

        // TAINT: search and category from query params
        // Passed to repository which uses them in SQL
        MealRepository::list_food_items(
            pool,
            user_id,
            search.as_deref(),
            category.as_deref(),
            limit,
            offset,
        )
        .await
    }

    // ==================== Meals ====================

    /// Create a meal entry with validation and nutrition calculation.
    ///
    /// TAINT TRANSFORM: HTTP request -> validate -> calculate nutrition -> repository
    /// Multi-hop cross-file flow:
    /// 1. handlers/meals.rs receives CreateMealRequest (SOURCE)
    /// 2. Here: validate and calculate nutrition (TRANSFORM)
    /// 3. MealRepository::create_meal executes SQL (SINK)
    pub async fn create_meal(
        pool: &DbPool,
        user_id: &str,
        request: CreateMealRequest,
    ) -> Result<MealWithTotals, AppError> {
        // TAINT TRANSFORM: Validate HTTP input
        Validators::validate_meal(&request)?;

        // Validate meal type
        let _meal_type = MealType::from_str(&request.meal_type)
            .ok_or(AppError::Validation("Invalid meal type".to_string()))?;

        // Calculate nutrition from food item if provided
        let calculated_nutrition = if let Some(ref food_item_id) = request.food_item_id {
            // TAINT FLOW: food_item_id -> repository query
            let food_item = MealRepository::find_food_item_by_id(pool, food_item_id)
                .await?
                .ok_or(AppError::NotFound("Food item not found".to_string()))?;

            // Verify ownership
            if food_item.user_id != user_id {
                return Err(AppError::Forbidden("Access to food item denied".to_string()));
            }

            // Calculate based on servings
            // TAINT TRANSFORM: HTTP servings * food item nutrition
            let servings = request.servings.unwrap_or(1.0);
            Some((
                (food_item.calories_per_serving as f64 * servings).round() as i32,
                food_item.protein_grams * servings,
                food_item.carbs_grams * servings,
                food_item.fat_grams * servings,
            ))
        } else {
            None
        };

        // TAINT FLOW: request + calculated -> MealRepository -> SQL INSERT
        let meal = MealRepository::create_meal(pool, user_id, &request, calculated_nutrition).await?;

        Ok(MealWithTotals::from(meal))
    }

    /// Get meal by ID with ownership check.
    pub async fn get_meal(
        pool: &DbPool,
        user_id: &str,
        meal_id: &str,
    ) -> Result<MealWithTotals, AppError> {
        // TAINT FLOW: meal_id -> repository query
        let meal = MealRepository::find_meal_by_id(pool, meal_id)
            .await?
            .ok_or(AppError::NotFound("Meal not found".to_string()))?;

        // Verify ownership
        if meal.user_id != user_id {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }

        Ok(MealWithTotals::from(meal))
    }

    /// Search meals with filters.
    ///
    /// TAINT TRANSFORM: query params -> validate dates -> repository
    pub async fn search_meals(
        pool: &DbPool,
        user_id: &str,
        query: MealSearchQuery,
    ) -> Result<Vec<MealWithTotals>, AppError> {
        // TAINT TRANSFORM: Validate and normalize query params
        let mut validated_query = query;

        // Validate meal type if provided
        if let Some(ref mt) = validated_query.meal_type {
            if MealType::from_str(mt).is_none() {
                return Err(AppError::Validation("Invalid meal type".to_string()));
            }
        }

        // Validate date format if provided
        if let Some(ref date) = validated_query.start_date {
            Validators::validate_date_format(date)?;
        }
        if let Some(ref date) = validated_query.end_date {
            Validators::validate_date_format(date)?;
        }

        // TAINT FLOW: validated query -> MealRepository -> dynamic SQL
        let meals = MealRepository::search_meals(pool, user_id, &validated_query).await?;

        Ok(meals.into_iter().map(MealWithTotals::from).collect())
    }

    /// Update a meal.
    ///
    /// TAINT TRANSFORM: HTTP request -> validate -> repository
    pub async fn update_meal(
        pool: &DbPool,
        user_id: &str,
        meal_id: &str,
        request: UpdateMealRequest,
    ) -> Result<MealWithTotals, AppError> {
        // Verify ownership first
        let existing = MealRepository::find_meal_by_id(pool, meal_id)
            .await?
            .ok_or(AppError::NotFound("Meal not found".to_string()))?;

        if existing.user_id != user_id {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }

        // Validate meal type if provided
        if let Some(ref mt) = request.meal_type {
            if MealType::from_str(mt).is_none() {
                return Err(AppError::Validation("Invalid meal type".to_string()));
            }
        }

        // TAINT FLOW: request -> MealRepository -> SQL UPDATE
        let meal = MealRepository::update_meal(pool, meal_id, user_id, &request).await?;

        Ok(MealWithTotals::from(meal))
    }

    /// Delete a meal.
    pub async fn delete_meal(
        pool: &DbPool,
        user_id: &str,
        meal_id: &str,
    ) -> Result<bool, AppError> {
        // TAINT FLOW: meal_id + user_id -> repository -> SQL DELETE
        MealRepository::delete_meal(pool, meal_id, user_id).await
    }

    /// Get daily nutrition totals.
    ///
    /// TAINT TRANSFORM: date param -> repository query -> aggregated result
    pub async fn get_daily_nutrition(
        pool: &DbPool,
        user_id: &str,
        date: &str,
    ) -> Result<NutritionInfo, AppError> {
        // TAINT FLOW: date -> MealRepository -> SQL aggregation
        let (calories, protein, carbs, fat, _count) =
            MealRepository::get_daily_totals(pool, user_id, date).await?;

        Ok(NutritionInfo {
            calories,
            protein_grams: protein,
            carbs_grams: carbs,
            fat_grams: fat,
        })
    }

    /// Get meals for a specific date.
    pub async fn get_meals_for_date(
        pool: &DbPool,
        user_id: &str,
        date: &str,
    ) -> Result<Vec<MealWithTotals>, AppError> {
        // TAINT FLOW: date -> search query -> repository
        let query = MealSearchQuery {
            meal_type: None,
            start_date: Some(format!("{}T00:00:00", date)),
            end_date: Some(format!("{}T23:59:59", date)),
            min_calories: None,
            max_calories: None,
            search: None,
            page: Some(0),
            per_page: Some(100),
        };

        Self::search_meals(pool, user_id, query).await
    }
}
