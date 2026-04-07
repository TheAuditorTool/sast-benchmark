//! Meal repository - SQL queries for meal and food item management.
//!
//! TAINT SINK: All methods receive data that originated from HTTP requests.
//! Multi-hop: HTTP -> handlers -> services -> repositories -> SQL
//!
//! Cross-file flow example:
//! 1. handlers/meals.rs: CreateMealRequest received from web::Json (SOURCE)
//! 2. services/meal_service.rs: validate and transform data (TRANSFORM)
//! 3. repositories/meal_repo.rs: execute SQL query (SINK)

use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::{
    CreateFoodItemRequest, CreateMealRequest, FoodItem, Meal, MealSearchQuery,
    UpdateMealRequest,
};
use sqlx::Row;
use uuid::Uuid;

/// Repository for meal and food item database operations
pub struct MealRepository;

impl MealRepository {
    // ==================== Food Items ====================

    /// Create a food item in the catalog.
    ///
    /// TAINT SINK: All fields from CreateFoodItemRequest flow to INSERT
    pub async fn create_food_item(
        pool: &DbPool,
        user_id: &str,
        request: &CreateFoodItemRequest,
    ) -> Result<FoodItem, AppError> {
        let id = Uuid::new_v4().to_string();

        // TAINT SINK: sqlx::query with tainted food item data
        sqlx::query(
            r#"
            INSERT INTO food_items (
                id, user_id, name, calories_per_serving, protein_grams,
                carbs_grams, fat_grams, serving_size, category, created_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'))
            "#,
        )
        .bind(&id)
        .bind(user_id)
        .bind(&request.name)                               // TAINT: from HTTP
        .bind(request.calories_per_serving)                // TAINT: from HTTP
        .bind(request.protein_grams.unwrap_or(0.0))        // TAINT: from HTTP
        .bind(request.carbs_grams.unwrap_or(0.0))          // TAINT: from HTTP
        .bind(request.fat_grams.unwrap_or(0.0))            // TAINT: from HTTP
        .bind(request.serving_size.as_deref().unwrap_or("1 serving"))  // TAINT
        .bind(request.category.as_deref().unwrap_or("other"))          // TAINT
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Self::find_food_item_by_id(pool, &id)
            .await?
            .ok_or(AppError::NotFound("Food item not found".to_string()))
    }

    /// Find food item by ID.
    pub async fn find_food_item_by_id(
        pool: &DbPool,
        id: &str,
    ) -> Result<Option<FoodItem>, AppError> {
        // TAINT SINK: id in WHERE clause
        let item = sqlx::query_as::<_, FoodItem>(
            "SELECT * FROM food_items WHERE id = ?",
        )
        .bind(id)  // TAINT: from path param
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(item)
    }

    /// List user's food items with optional search.
    ///
    /// TAINT SINK: search term in LIKE clause
    pub async fn list_food_items(
        pool: &DbPool,
        user_id: &str,
        search: Option<&str>,
        category: Option<&str>,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<FoodItem>, AppError> {
        let mut query = String::from(
            "SELECT * FROM food_items WHERE user_id = ?"
        );
        let mut bind_values: Vec<String> = vec![user_id.to_string()];

        // TAINT: search term in dynamic query
        if let Some(search_term) = search {
            query.push_str(" AND name LIKE ?");
            bind_values.push(format!("%{}%", search_term));  // TAINT: from query param
        }

        // TAINT: category in dynamic query
        if let Some(cat) = category {
            query.push_str(" AND category = ?");
            bind_values.push(cat.to_string());  // TAINT: from query param
        }

        query.push_str(" ORDER BY name ASC LIMIT ? OFFSET ?");

        // Build and execute query
        let mut q = sqlx::query_as::<_, FoodItem>(&query);
        for val in &bind_values {
            q = q.bind(val);
        }
        q = q.bind(limit).bind(offset);

        let items = q
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(items)
    }

    // ==================== Meals ====================

    /// Create a meal entry.
    ///
    /// TAINT SINK: All fields from CreateMealRequest flow to INSERT
    /// This is the end of a multi-hop cross-file taint flow:
    /// handlers/meals.rs -> services/meal_service.rs -> here
    pub async fn create_meal(
        pool: &DbPool,
        user_id: &str,
        request: &CreateMealRequest,
        calculated_nutrition: Option<(i32, f64, f64, f64)>,
    ) -> Result<Meal, AppError> {
        let id = Uuid::new_v4().to_string();

        // Use calculated nutrition if provided, otherwise use request values
        let (calories, protein, carbs, fat) = calculated_nutrition.unwrap_or((
            request.calories.unwrap_or(0),
            request.protein_grams.unwrap_or(0.0),
            request.carbs_grams.unwrap_or(0.0),
            request.fat_grams.unwrap_or(0.0),
        ));

        // TAINT SINK: All values derived from HTTP request
        sqlx::query(
            r#"
            INSERT INTO meals (
                id, user_id, food_item_id, name, calories, protein_grams,
                carbs_grams, fat_grams, servings, meal_type, consumed_at,
                notes, created_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'))
            "#,
        )
        .bind(&id)
        .bind(user_id)
        .bind(&request.food_item_id)              // TAINT: from HTTP (optional FK)
        .bind(&request.name)                       // TAINT: from HTTP
        .bind(calories)                            // TAINT: calculated from HTTP data
        .bind(protein)                             // TAINT: from HTTP
        .bind(carbs)                               // TAINT: from HTTP
        .bind(fat)                                 // TAINT: from HTTP
        .bind(request.servings.unwrap_or(1.0))    // TAINT: from HTTP
        .bind(&request.meal_type)                  // TAINT: from HTTP
        .bind(&request.consumed_at)                // TAINT: from HTTP
        .bind(&request.notes)                      // TAINT: from HTTP
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Self::find_meal_by_id(pool, &id)
            .await?
            .ok_or(AppError::NotFound("Meal not found".to_string()))
    }

    /// Find meal by ID.
    pub async fn find_meal_by_id(pool: &DbPool, id: &str) -> Result<Option<Meal>, AppError> {
        let meal = sqlx::query_as::<_, Meal>("SELECT * FROM meals WHERE id = ?")
            .bind(id)  // TAINT: from path param
            .fetch_optional(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(meal)
    }

    // vuln-code-snippet start sqliCalorieSearchMeals
    /// Search meals with filters.
    ///
    /// TAINT SINK: All MealSearchQuery fields flow to dynamic SQL
    /// Demonstrates complex cross-file taint flow with multiple params
    pub async fn search_meals(
        pool: &DbPool,
        user_id: &str,
        query_params: &MealSearchQuery,
    ) -> Result<Vec<Meal>, AppError> {
        let mut query = String::from("SELECT * FROM meals WHERE user_id = ?");
        let mut bindings: Vec<Box<dyn ToString + Send + Sync>> = vec![Box::new(user_id.to_string())];

        // TAINT: meal_type from query params
        if let Some(ref meal_type) = query_params.meal_type {
            query.push_str(" AND meal_type = ?");
            bindings.push(Box::new(meal_type.clone()));
        }

        // TAINT: date range from query params
        if let Some(ref start) = query_params.start_date {
            query.push_str(" AND consumed_at >= ?");
            bindings.push(Box::new(start.clone()));
        }

        if let Some(ref end) = query_params.end_date {
            query.push_str(" AND consumed_at <= ?");
            bindings.push(Box::new(end.clone()));
        }

        // TAINT: calorie range from query params
        if let Some(min_cal) = query_params.min_calories {
            query.push_str(" AND calories >= ?");
            bindings.push(Box::new(min_cal.to_string()));
        }

        if let Some(max_cal) = query_params.max_calories {
            query.push_str(" AND calories <= ?");
            bindings.push(Box::new(max_cal.to_string()));
        }

        // TAINT: search term in LIKE (requires parameterized query)
        if let Some(ref search) = query_params.search {
            query.push_str(" AND name LIKE ?");
            bindings.push(Box::new(format!("%{}%", search)));
        }

        query.push_str(" ORDER BY consumed_at DESC");

        // Pagination - TAINT: limit/offset from query params
        let limit = query_params.per_page.unwrap_or(20).min(100);
        let offset = query_params.page.unwrap_or(0) * limit;
        query.push_str(" LIMIT ? OFFSET ?");

        // Execute with bindings
        let mut q = sqlx::query_as::<_, Meal>(&query);
        for binding in &bindings {
            // vuln-code-snippet target-line sqliCalorieSearchMeals
            q = q.bind(binding.to_string());
        }
        q = q.bind(limit).bind(offset);

        let meals = q
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(meals)
    }
    // vuln-code-snippet end sqliCalorieSearchMeals

    // vuln-code-snippet start sqliCalorieUpdateMeal
    /// Update a meal.
    ///
    /// TAINT SINK: UpdateMealRequest fields flow to UPDATE statement
    pub async fn update_meal(
        pool: &DbPool,
        meal_id: &str,
        user_id: &str,
        request: &UpdateMealRequest,
    ) -> Result<Meal, AppError> {
        // Build dynamic UPDATE query
        let mut updates = Vec::new();
        let mut values: Vec<String> = Vec::new();

        if let Some(ref name) = request.name {
            updates.push("name = ?");
            values.push(name.clone());  // TAINT
        }
        if let Some(calories) = request.calories {
            updates.push("calories = ?");
            values.push(calories.to_string());  // TAINT
        }
        if let Some(protein) = request.protein_grams {
            updates.push("protein_grams = ?");
            values.push(protein.to_string());  // TAINT
        }
        if let Some(carbs) = request.carbs_grams {
            updates.push("carbs_grams = ?");
            values.push(carbs.to_string());  // TAINT
        }
        if let Some(fat) = request.fat_grams {
            updates.push("fat_grams = ?");
            values.push(fat.to_string());  // TAINT
        }
        if let Some(servings) = request.servings {
            updates.push("servings = ?");
            values.push(servings.to_string());  // TAINT
        }
        if let Some(ref meal_type) = request.meal_type {
            updates.push("meal_type = ?");
            values.push(meal_type.clone());  // TAINT
        }
        if let Some(ref consumed_at) = request.consumed_at {
            updates.push("consumed_at = ?");
            values.push(consumed_at.clone());  // TAINT
        }
        if let Some(ref notes) = request.notes {
            updates.push("notes = ?");
            values.push(notes.clone());  // TAINT
        }

        if updates.is_empty() {
            return Self::find_meal_by_id(pool, meal_id)
                .await?
                .ok_or(AppError::NotFound("Meal not found".to_string()));
        }

        let query = format!(
            "UPDATE meals SET {} WHERE id = ? AND user_id = ?",
            updates.join(", ")
        );

        let mut q = sqlx::query(&query);
        for value in &values {
            // vuln-code-snippet target-line sqliCalorieUpdateMeal
            q = q.bind(value);
        }
        q = q.bind(meal_id).bind(user_id);

        let result = q
            .execute(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Meal not found or access denied".to_string()));
        }

        Self::find_meal_by_id(pool, meal_id)
            .await?
            .ok_or(AppError::NotFound("Meal not found".to_string()))
    }
    // vuln-code-snippet end sqliCalorieUpdateMeal

    /// Delete a meal.
    pub async fn delete_meal(
        pool: &DbPool,
        meal_id: &str,
        user_id: &str,
    ) -> Result<bool, AppError> {
        // TAINT SINK: meal_id and user_id in DELETE WHERE
        let result = sqlx::query("DELETE FROM meals WHERE id = ? AND user_id = ?")
            .bind(meal_id)   // TAINT: from path param
            .bind(user_id)   // TAINT: from auth context
            .execute(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    /// Get daily meal totals.
    ///
    /// TAINT SINK: date in WHERE clause
    pub async fn get_daily_totals(
        pool: &DbPool,
        user_id: &str,
        date: &str,
    ) -> Result<(i32, f64, f64, f64, i32), AppError> {
        // TAINT SINK: date param flows to SQL
        let row = sqlx::query(
            r#"
            SELECT
                COALESCE(SUM(calories * servings), 0) as total_calories,
                COALESCE(SUM(protein_grams * servings), 0.0) as total_protein,
                COALESCE(SUM(carbs_grams * servings), 0.0) as total_carbs,
                COALESCE(SUM(fat_grams * servings), 0.0) as total_fat,
                COUNT(*) as meal_count
            FROM meals
            WHERE user_id = ? AND date(consumed_at) = date(?)
            "#,
        )
        .bind(user_id)
        .bind(date)  // TAINT: from query param or calculated
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok((
            row.get::<i32, _>("total_calories"),
            row.get::<f64, _>("total_protein"),
            row.get::<f64, _>("total_carbs"),
            row.get::<f64, _>("total_fat"),
            row.get::<i32, _>("meal_count"),
        ))
    }
}
