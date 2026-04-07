//! Domain models for the calorie tracking application.
//!
//! All models use the `validator` crate for input validation (Rust's Joi/Zod equivalent).
//! TAINT FLOW: HTTP input -> model deserialization -> validation -> business logic

pub mod food_item;
pub mod meal;
pub mod schedule;
pub mod user;
pub mod workout;

pub use food_item::*;
pub use meal::*;
pub use schedule::*;
pub use user::*;
pub use workout::*;

use serde::{Deserialize, Serialize};

/// Standard API response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            message: None,
        }
    }

    pub fn success_with_message(data: T, message: &str) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            message: Some(message.to_string()),
        }
    }

    pub fn error(error: &str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error.to_string()),
            message: None,
        }
    }
}

/// Pagination parameters
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

impl PaginationParams {
    pub fn offset(&self) -> u32 {
        let page = self.page.unwrap_or(1).max(1);
        let per_page = self.per_page();
        (page - 1) * per_page
    }

    pub fn per_page(&self) -> u32 {
        self.per_page.unwrap_or(20).min(100)
    }
}

/// Date range filter for queries
#[derive(Debug, Deserialize)]
pub struct DateRangeFilter {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

/// Daily summary statistics
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DailySummary {
    pub id: String,
    pub user_id: String,
    pub date: String,
    pub total_calories_consumed: i32,
    pub total_calories_burned: i32,
    pub net_calories: i32,
    pub total_protein_grams: f64,
    pub total_carbs_grams: f64,
    pub total_fat_grams: f64,
    pub meal_count: i32,
    pub workout_count: i32,
    pub goal_met: bool,
}

/// Nutrition breakdown
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct NutritionInfo {
    pub calories: i32,
    pub protein_grams: f64,
    pub carbs_grams: f64,
    pub fat_grams: f64,
}

impl NutritionInfo {
    pub fn scale(&self, factor: f64) -> Self {
        Self {
            calories: (self.calories as f64 * factor).round() as i32,
            protein_grams: self.protein_grams * factor,
            carbs_grams: self.carbs_grams * factor,
            fat_grams: self.fat_grams * factor,
        }
    }

    pub fn add(&mut self, other: &NutritionInfo) {
        self.calories += other.calories;
        self.protein_grams += other.protein_grams;
        self.carbs_grams += other.carbs_grams;
        self.fat_grams += other.fat_grams;
    }
}
