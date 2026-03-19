//! Validators - Joi/Zod equivalent validation logic.
//!
//! TAINT TRANSFORM: Validates and sanitizes all user input before
//! it reaches the service/repository layers.

use regex::Regex;
use once_cell::sync::Lazy;

use crate::errors::AppError;
use crate::models::{
    CreateUserRequest, LoginRequest, ChangePasswordRequest,
    CreateMealRequest, CreateFoodItemRequest,
    CreateWorkoutRequest, CreateExerciseTypeRequest,
    CreateScheduleRequest,
};

// Compiled regex patterns (Zod-like schema definitions)
static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
});

static DATE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
});

static TIME_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\d{2}:\d{2}(:\d{2})?$").unwrap()
});

static UUID_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$").unwrap()
});

/// Validators struct - Joi/Zod-like validation schemas
pub struct Validators;

impl Validators {
    // ==================== User Validation ====================

    /// Validate user registration request
    /// TAINT TRANSFORM: Validates email, password, username from HTTP body
    pub fn validate_user_registration(request: &CreateUserRequest) -> Result<(), AppError> {
        // Email validation (Zod: z.string().email())
        if !EMAIL_REGEX.is_match(&request.email) {
            return Err(AppError::Validation("Invalid email format".to_string()));
        }

        // Username validation (Zod: z.string().min(3).max(50))
        if request.username.len() < 3 || request.username.len() > 50 {
            return Err(AppError::Validation(
                "Username must be 3-50 characters".to_string(),
            ));
        }

        // Username allowed characters
        if !request.username.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(AppError::Validation(
                "Username can only contain letters, numbers, and underscores".to_string(),
            ));
        }

        // Password validation (Zod: z.string().min(8).max(100))
        Self::validate_password_strength(&request.password)?;

        // Optional height/weight validation
        if let Some(height) = request.height_cm {
            if height < 50.0 || height > 300.0 {
                return Err(AppError::Validation(
                    "Height must be between 50 and 300 cm".to_string(),
                ));
            }
        }

        if let Some(weight) = request.weight_kg {
            if weight < 20.0 || weight > 500.0 {
                return Err(AppError::Validation(
                    "Weight must be between 20 and 500 kg".to_string(),
                ));
            }
        }

        if let Some(age) = request.age {
            if age < 13 || age > 120 {
                return Err(AppError::Validation(
                    "Age must be between 13 and 120".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Validate login request
    /// TAINT TRANSFORM: Validates credentials from HTTP body
    pub fn validate_login(request: &LoginRequest) -> Result<(), AppError> {
        if !EMAIL_REGEX.is_match(&request.email) {
            return Err(AppError::Validation("Invalid email format".to_string()));
        }

        if request.password.is_empty() {
            return Err(AppError::Validation("Password is required".to_string()));
        }

        Ok(())
    }

    /// Validate password change request
    /// TAINT TRANSFORM: Validates both passwords from HTTP body
    pub fn validate_password_change(request: &ChangePasswordRequest) -> Result<(), AppError> {
        if request.current_password.is_empty() {
            return Err(AppError::Validation(
                "Current password is required".to_string(),
            ));
        }

        Self::validate_password_strength(&request.new_password)?;

        if request.current_password == request.new_password {
            return Err(AppError::Validation(
                "New password must be different from current password".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate password strength
    fn validate_password_strength(password: &str) -> Result<(), AppError> {
        if password.len() < 8 {
            return Err(AppError::Validation(
                "Password must be at least 8 characters".to_string(),
            ));
        }

        if password.len() > 100 {
            return Err(AppError::Validation(
                "Password must be at most 100 characters".to_string(),
            ));
        }

        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());

        if !has_uppercase || !has_lowercase || !has_digit {
            return Err(AppError::Validation(
                "Password must contain uppercase, lowercase, and a digit".to_string(),
            ));
        }

        Ok(())
    }

    // ==================== Meal Validation ====================

    /// Validate meal creation request
    /// TAINT TRANSFORM: Validates meal data from HTTP body
    pub fn validate_meal(request: &CreateMealRequest) -> Result<(), AppError> {
        // Name validation (Zod: z.string().min(1).max(200))
        if request.name.is_empty() || request.name.len() > 200 {
            return Err(AppError::Validation(
                "Meal name must be 1-200 characters".to_string(),
            ));
        }

        // Calories validation (Zod: z.number().min(0).max(50000).optional())
        if let Some(calories) = request.calories {
            if calories < 0 || calories > 50000 {
                return Err(AppError::Validation(
                    "Calories must be between 0 and 50000".to_string(),
                ));
            }
        }

        // Macro validation
        if let Some(protein) = request.protein_grams {
            if protein < 0.0 || protein > 1000.0 {
                return Err(AppError::Validation(
                    "Protein must be between 0 and 1000 grams".to_string(),
                ));
            }
        }

        if let Some(carbs) = request.carbs_grams {
            if carbs < 0.0 || carbs > 2000.0 {
                return Err(AppError::Validation(
                    "Carbs must be between 0 and 2000 grams".to_string(),
                ));
            }
        }

        if let Some(fat) = request.fat_grams {
            if fat < 0.0 || fat > 500.0 {
                return Err(AppError::Validation(
                    "Fat must be between 0 and 500 grams".to_string(),
                ));
            }
        }

        // Servings validation
        if let Some(servings) = request.servings {
            if servings <= 0.0 || servings > 100.0 {
                return Err(AppError::Validation(
                    "Servings must be between 0.01 and 100".to_string(),
                ));
            }
        }

        // Meal type validation
        let valid_types = ["breakfast", "lunch", "dinner", "snack", "other"];
        if !valid_types.contains(&request.meal_type.to_lowercase().as_str()) {
            return Err(AppError::Validation(
                "Meal type must be breakfast, lunch, dinner, snack, or other".to_string(),
            ));
        }

        // Consumed at validation - must be valid datetime
        if request.consumed_at.is_empty() {
            return Err(AppError::Validation(
                "Consumed at datetime is required".to_string(),
            ));
        }

        // Notes length
        if let Some(ref notes) = request.notes {
            if notes.len() > 500 {
                return Err(AppError::Validation(
                    "Notes must be at most 500 characters".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Validate food item creation request
    /// TAINT TRANSFORM: Validates food item data from HTTP body
    pub fn validate_food_item(request: &CreateFoodItemRequest) -> Result<(), AppError> {
        if request.name.is_empty() || request.name.len() > 200 {
            return Err(AppError::Validation(
                "Food item name must be 1-200 characters".to_string(),
            ));
        }

        if request.calories_per_serving < 0 || request.calories_per_serving > 10000 {
            return Err(AppError::Validation(
                "Calories per serving must be between 0 and 10000".to_string(),
            ));
        }

        // Serving size string validation
        if let Some(ref serving_size) = request.serving_size {
            if serving_size.is_empty() || serving_size.len() > 100 {
                return Err(AppError::Validation(
                    "Serving size must be 1-100 characters".to_string(),
                ));
            }
        }

        // Category validation
        if let Some(ref category) = request.category {
            if category.len() > 50 {
                return Err(AppError::Validation(
                    "Category must be at most 50 characters".to_string(),
                ));
            }
        }

        // Macro validation
        if let Some(protein) = request.protein_grams {
            if protein < 0.0 || protein > 500.0 {
                return Err(AppError::Validation(
                    "Protein per serving must be between 0 and 500 grams".to_string(),
                ));
            }
        }

        if let Some(carbs) = request.carbs_grams {
            if carbs < 0.0 || carbs > 500.0 {
                return Err(AppError::Validation(
                    "Carbs per serving must be between 0 and 500 grams".to_string(),
                ));
            }
        }

        if let Some(fat) = request.fat_grams {
            if fat < 0.0 || fat > 200.0 {
                return Err(AppError::Validation(
                    "Fat per serving must be between 0 and 200 grams".to_string(),
                ));
            }
        }

        Ok(())
    }

    // ==================== Workout Validation ====================

    /// Validate workout creation request
    /// TAINT TRANSFORM: Validates workout data from HTTP body
    pub fn validate_workout(request: &CreateWorkoutRequest) -> Result<(), AppError> {
        // Name validation
        if request.name.is_empty() || request.name.len() > 200 {
            return Err(AppError::Validation(
                "Workout name must be 1-200 characters".to_string(),
            ));
        }

        // Duration validation (Zod: z.number().min(1).max(1440))
        if request.duration_minutes < 1 || request.duration_minutes > 1440 {
            return Err(AppError::Validation(
                "Duration must be between 1 and 1440 minutes".to_string(),
            ));
        }

        // Calories burned validation
        if let Some(calories) = request.calories_burned {
            if calories < 0 || calories > 50000 {
                return Err(AppError::Validation(
                    "Calories burned must be between 0 and 50000".to_string(),
                ));
            }
        }

        // Intensity validation (string: light, moderate, intense, extreme)
        if let Some(ref intensity) = request.intensity {
            let valid_intensities = ["light", "moderate", "intense", "extreme"];
            if !valid_intensities.contains(&intensity.to_lowercase().as_str()) {
                return Err(AppError::Validation(
                    "Intensity must be light, moderate, intense, or extreme".to_string(),
                ));
            }
        }

        // Performed at validation - required datetime
        if request.performed_at.is_empty() {
            return Err(AppError::Validation(
                "Performed at datetime is required".to_string(),
            ));
        }

        // Heart rate validation
        if let Some(hr) = request.heart_rate_avg {
            if hr < 30 || hr > 250 {
                return Err(AppError::Validation(
                    "Heart rate must be between 30 and 250 bpm".to_string(),
                ));
            }
        }

        // Notes length
        if let Some(ref notes) = request.notes {
            if notes.len() > 500 {
                return Err(AppError::Validation(
                    "Notes must be at most 500 characters".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Validate exercise type creation request
    pub fn validate_exercise_type(request: &CreateExerciseTypeRequest) -> Result<(), AppError> {
        if request.name.is_empty() || request.name.len() > 200 {
            return Err(AppError::Validation(
                "Exercise type name must be 1-200 characters".to_string(),
            ));
        }

        if request.calories_per_minute < 1 || request.calories_per_minute > 100 {
            return Err(AppError::Validation(
                "Calories per minute must be between 1 and 100".to_string(),
            ));
        }

        if let Some(ref category) = request.category {
            let valid_categories = ["cardio", "strength", "flexibility", "sports", "other"];
            if !valid_categories.contains(&category.to_lowercase().as_str()) {
                return Err(AppError::Validation(
                    "Category must be cardio, strength, flexibility, sports, or other".to_string(),
                ));
            }
        }

        if let Some(met) = request.met_value {
            if met < 1.0 || met > 25.0 {
                return Err(AppError::Validation(
                    "MET value must be between 1 and 25".to_string(),
                ));
            }
        }

        Ok(())
    }

    // ==================== Schedule Validation ====================

    /// Validate schedule creation request
    /// TAINT TRANSFORM: Validates schedule data from HTTP body
    pub fn validate_schedule(request: &CreateScheduleRequest) -> Result<(), AppError> {
        // Day of week validation (Zod: z.number().min(0).max(6))
        if request.day_of_week < 0 || request.day_of_week > 6 {
            return Err(AppError::Validation(
                "Day of week must be 0-6 (Sunday=0, Saturday=6)".to_string(),
            ));
        }

        // Target calories validation
        if request.target_calories < 500 || request.target_calories > 10000 {
            return Err(AppError::Validation(
                "Target calories must be between 500 and 10000".to_string(),
            ));
        }

        // Macro targets validation (f64)
        if let Some(protein) = request.target_protein_grams {
            if protein < 0.0 || protein > 500.0 {
                return Err(AppError::Validation(
                    "Target protein must be between 0 and 500 grams".to_string(),
                ));
            }
        }

        if let Some(carbs) = request.target_carbs_grams {
            if carbs < 0.0 || carbs > 1000.0 {
                return Err(AppError::Validation(
                    "Target carbs must be between 0 and 1000 grams".to_string(),
                ));
            }
        }

        if let Some(fat) = request.target_fat_grams {
            if fat < 0.0 || fat > 500.0 {
                return Err(AppError::Validation(
                    "Target fat must be between 0 and 500 grams".to_string(),
                ));
            }
        }

        // Planned workouts length
        if let Some(ref workouts) = request.planned_workouts {
            if workouts.len() > 2000 {
                return Err(AppError::Validation(
                    "Planned workouts must be at most 2000 characters".to_string(),
                ));
            }
        }

        // Notes length
        if let Some(ref notes) = request.notes {
            if notes.len() > 500 {
                return Err(AppError::Validation(
                    "Notes must be at most 500 characters".to_string(),
                ));
            }
        }

        Ok(())
    }

    // ==================== Common Validators ====================

    /// Validate date format (YYYY-MM-DD)
    /// TAINT TRANSFORM: Validates date string format
    pub fn validate_date_format(date: &str) -> Result<(), AppError> {
        if !DATE_REGEX.is_match(date) {
            return Err(AppError::Validation(
                "Date must be in YYYY-MM-DD format".to_string(),
            ));
        }

        // Also validate it's a real date
        if chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").is_err() {
            return Err(AppError::Validation("Invalid date".to_string()));
        }

        Ok(())
    }

    /// Validate time format (HH:MM or HH:MM:SS)
    /// TAINT TRANSFORM: Validates time string format
    pub fn validate_time_format(time: &str) -> Result<(), AppError> {
        if !TIME_REGEX.is_match(time) {
            return Err(AppError::Validation(
                "Time must be in HH:MM or HH:MM:SS format".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate UUID format
    /// TAINT TRANSFORM: Validates UUID string
    pub fn validate_uuid(id: &str) -> Result<(), AppError> {
        if !UUID_REGEX.is_match(id) {
            return Err(AppError::Validation("Invalid ID format".to_string()));
        }

        Ok(())
    }

    /// Validate pagination parameters
    pub fn validate_pagination(page: Option<i64>, per_page: Option<i64>) -> Result<(i64, i64), AppError> {
        let page = page.unwrap_or(0);
        let per_page = per_page.unwrap_or(20);

        if page < 0 {
            return Err(AppError::Validation("Page must be non-negative".to_string()));
        }

        if per_page < 1 || per_page > 100 {
            return Err(AppError::Validation(
                "Per page must be between 1 and 100".to_string(),
            ));
        }

        Ok((page, per_page))
    }
}
