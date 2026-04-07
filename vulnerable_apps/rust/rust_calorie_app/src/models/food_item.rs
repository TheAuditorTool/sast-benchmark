//! Food item model - re-exports from meal.rs for convenience.
//!
//! Food items are the reusable catalog of foods that can be referenced by meals.

pub use super::meal::{CreateFoodItemRequest, FoodItem};

use serde::{Deserialize, Serialize};
use validator::Validate;

/// TAINT SOURCE: Update food item request from HTTP
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateFoodItemRequest {
    #[validate(length(min = 1, max = 200, message = "Name must be 1-200 characters"))]
    pub name: Option<String>,

    #[validate(range(min = 0, max = 10000, message = "Calories must be 0-10000"))]
    pub calories_per_serving: Option<i32>,

    #[validate(range(min = 0.0, max = 1000.0))]
    pub protein_grams: Option<f64>,

    #[validate(range(min = 0.0, max = 1000.0))]
    pub carbs_grams: Option<f64>,

    #[validate(range(min = 0.0, max = 1000.0))]
    pub fat_grams: Option<f64>,

    #[validate(length(max = 100))]
    pub serving_size: Option<String>,

    #[validate(length(max = 50))]
    pub category: Option<String>,
}

/// TAINT SOURCE: Search food items query params
#[derive(Debug, Deserialize)]
pub struct FoodItemSearchQuery {
    pub category: Option<String>,
    pub search: Option<String>,
    pub min_calories: Option<i32>,
    pub max_calories: Option<i32>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

/// Food item with usage statistics
#[derive(Debug, Serialize)]
pub struct FoodItemWithStats {
    #[serde(flatten)]
    pub food_item: FoodItem,
    pub times_used: i32,
    pub last_used: Option<String>,
}

/// Nutritional breakdown for display
#[derive(Debug, Serialize)]
pub struct NutritionalBreakdown {
    pub calories: i32,
    pub protein_grams: f64,
    pub carbs_grams: f64,
    pub fat_grams: f64,
    pub protein_percent: f64,
    pub carbs_percent: f64,
    pub fat_percent: f64,
}

impl NutritionalBreakdown {
    pub fn from_macros(calories: i32, protein: f64, carbs: f64, fat: f64) -> Self {
        // Calculate percentages based on calorie contribution
        // Protein: 4 cal/g, Carbs: 4 cal/g, Fat: 9 cal/g
        let protein_cals = protein * 4.0;
        let carbs_cals = carbs * 4.0;
        let fat_cals = fat * 9.0;
        let total_cals = protein_cals + carbs_cals + fat_cals;

        let (protein_pct, carbs_pct, fat_pct) = if total_cals > 0.0 {
            (
                (protein_cals / total_cals) * 100.0,
                (carbs_cals / total_cals) * 100.0,
                (fat_cals / total_cals) * 100.0,
            )
        } else {
            (0.0, 0.0, 0.0)
        };

        Self {
            calories,
            protein_grams: protein,
            carbs_grams: carbs,
            fat_grams: fat,
            protein_percent: protein_pct,
            carbs_percent: carbs_pct,
            fat_percent: fat_pct,
        }
    }
}

/// Common food categories
pub const FOOD_CATEGORIES: &[&str] = &[
    "fruits",
    "vegetables",
    "grains",
    "protein",
    "dairy",
    "fats",
    "sweets",
    "beverages",
    "fast_food",
    "restaurant",
    "homemade",
    "other",
];

/// Validate food category
pub fn is_valid_category(category: &str) -> bool {
    FOOD_CATEGORIES.contains(&category.to_lowercase().as_str())
}
