//! Meal and food item models with validation.
//!
//! TAINT FLOW: HTTP request -> CreateMealRequest -> MealService -> MealRepository -> SQL

use serde::{Deserialize, Serialize};
use validator::Validate;

/// Meal types enum
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MealType {
    Breakfast,
    Lunch,
    Dinner,
    Snack,
}

impl MealType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MealType::Breakfast => "breakfast",
            MealType::Lunch => "lunch",
            MealType::Dinner => "dinner",
            MealType::Snack => "snack",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "breakfast" => Some(MealType::Breakfast),
            "lunch" => Some(MealType::Lunch),
            "dinner" => Some(MealType::Dinner),
            "snack" => Some(MealType::Snack),
            _ => None,
        }
    }
}

/// Food item entity (reusable food catalog)
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct FoodItem {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub calories_per_serving: i32,
    pub protein_grams: f64,
    pub carbs_grams: f64,
    pub fat_grams: f64,
    pub serving_size: String,
    pub category: String,
    pub created_at: String,
}

/// TAINT SOURCE: Create food item request from HTTP
#[derive(Debug, Deserialize, Validate)]
pub struct CreateFoodItemRequest {
    #[validate(length(min = 1, max = 200, message = "Name must be 1-200 characters"))]
    pub name: String,

    #[validate(range(min = 0, max = 10000, message = "Calories must be 0-10000"))]
    pub calories_per_serving: i32,

    #[validate(range(min = 0.0, max = 1000.0, message = "Protein must be 0-1000g"))]
    pub protein_grams: Option<f64>,

    #[validate(range(min = 0.0, max = 1000.0, message = "Carbs must be 0-1000g"))]
    pub carbs_grams: Option<f64>,

    #[validate(range(min = 0.0, max = 1000.0, message = "Fat must be 0-1000g"))]
    pub fat_grams: Option<f64>,

    #[validate(length(max = 100, message = "Serving size too long"))]
    pub serving_size: Option<String>,

    #[validate(length(max = 50, message = "Category too long"))]
    pub category: Option<String>,
}

/// Meal entry entity (actual consumption)
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Meal {
    pub id: String,
    pub user_id: String,
    pub food_item_id: Option<String>,
    pub name: String,
    pub calories: i32,
    pub protein_grams: f64,
    pub carbs_grams: f64,
    pub fat_grams: f64,
    pub servings: f64,
    pub meal_type: String,
    pub consumed_at: String,
    pub notes: Option<String>,
    pub created_at: String,
}

/// TAINT SOURCE: Create meal request from HTTP
/// This is a primary taint entry point for the application
#[derive(Debug, Deserialize, Validate)]
pub struct CreateMealRequest {
    /// Optional food item ID to copy nutrition from
    pub food_item_id: Option<String>,

    #[validate(length(min = 1, max = 200, message = "Name must be 1-200 characters"))]
    pub name: String,

    /// Calories - required if no food_item_id
    #[validate(range(min = 0, max = 50000, message = "Calories must be 0-50000"))]
    pub calories: Option<i32>,

    #[validate(range(min = 0.0, max = 1000.0, message = "Protein must be 0-1000g"))]
    pub protein_grams: Option<f64>,

    #[validate(range(min = 0.0, max = 1000.0, message = "Carbs must be 0-1000g"))]
    pub carbs_grams: Option<f64>,

    #[validate(range(min = 0.0, max = 1000.0, message = "Fat must be 0-1000g"))]
    pub fat_grams: Option<f64>,

    #[validate(range(min = 0.1, max = 100.0, message = "Servings must be 0.1-100"))]
    pub servings: Option<f64>,

    /// Meal type: breakfast, lunch, dinner, snack
    #[validate(length(min = 1, message = "Meal type required"))]
    pub meal_type: String,

    /// When the meal was consumed (ISO 8601 datetime)
    #[validate(length(min = 1, message = "Consumed at datetime required"))]
    pub consumed_at: String,

    #[validate(length(max = 500, message = "Notes too long"))]
    pub notes: Option<String>,
}

/// TAINT SOURCE: Update meal request from HTTP
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateMealRequest {
    #[validate(length(min = 1, max = 200, message = "Name must be 1-200 characters"))]
    pub name: Option<String>,

    #[validate(range(min = 0, max = 50000, message = "Calories must be 0-50000"))]
    pub calories: Option<i32>,

    #[validate(range(min = 0.0, max = 1000.0))]
    pub protein_grams: Option<f64>,

    #[validate(range(min = 0.0, max = 1000.0))]
    pub carbs_grams: Option<f64>,

    #[validate(range(min = 0.0, max = 1000.0))]
    pub fat_grams: Option<f64>,

    #[validate(range(min = 0.1, max = 100.0))]
    pub servings: Option<f64>,

    pub meal_type: Option<String>,

    pub consumed_at: Option<String>,

    #[validate(length(max = 500))]
    pub notes: Option<String>,
}

/// TAINT SOURCE: Search/filter meals query params
#[derive(Debug, Deserialize)]
pub struct MealSearchQuery {
    pub meal_type: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub min_calories: Option<i32>,
    pub max_calories: Option<i32>,
    pub search: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

/// Meal with calculated totals
#[derive(Debug, Serialize)]
pub struct MealWithTotals {
    #[serde(flatten)]
    pub meal: Meal,
    pub total_calories: i32,
    pub total_protein: f64,
    pub total_carbs: f64,
    pub total_fat: f64,
}

impl From<Meal> for MealWithTotals {
    fn from(meal: Meal) -> Self {
        let servings = meal.servings;
        Self {
            total_calories: (meal.calories as f64 * servings).round() as i32,
            total_protein: meal.protein_grams * servings,
            total_carbs: meal.carbs_grams * servings,
            total_fat: meal.fat_grams * servings,
            meal: meal,
        }
    }
}
