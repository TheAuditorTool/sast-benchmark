//! Workout and exercise models with validation.
//!
//! TAINT FLOW: HTTP request -> CreateWorkoutRequest -> WorkoutService -> WorkoutRepository -> SQL

use serde::{Deserialize, Serialize};
use validator::Validate;

/// Exercise intensity levels
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Intensity {
    Light,
    Moderate,
    Intense,
    Extreme,
}

impl Intensity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Intensity::Light => "light",
            Intensity::Moderate => "moderate",
            Intensity::Intense => "intense",
            Intensity::Extreme => "extreme",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "light" => Some(Intensity::Light),
            "moderate" => Some(Intensity::Moderate),
            "intense" => Some(Intensity::Intense),
            "extreme" => Some(Intensity::Extreme),
            _ => None,
        }
    }

    /// Calorie burn multiplier based on intensity
    pub fn calorie_multiplier(&self) -> f64 {
        match self {
            Intensity::Light => 0.75,
            Intensity::Moderate => 1.0,
            Intensity::Intense => 1.25,
            Intensity::Extreme => 1.5,
        }
    }
}

/// Exercise category
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExerciseCategory {
    Cardio,
    Strength,
    Flexibility,
    Sports,
    Other,
}

impl ExerciseCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            ExerciseCategory::Cardio => "cardio",
            ExerciseCategory::Strength => "strength",
            ExerciseCategory::Flexibility => "flexibility",
            ExerciseCategory::Sports => "sports",
            ExerciseCategory::Other => "other",
        }
    }
}

/// Exercise type entity (reusable exercise catalog)
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct ExerciseType {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub calories_per_minute: i32,
    pub category: String,
    pub met_value: f64,
    pub created_at: String,
}

/// TAINT SOURCE: Create exercise type request from HTTP
#[derive(Debug, Deserialize, Validate)]
pub struct CreateExerciseTypeRequest {
    #[validate(length(min = 1, max = 200, message = "Name must be 1-200 characters"))]
    pub name: String,

    #[validate(range(min = 1, max = 100, message = "Calories per minute must be 1-100"))]
    pub calories_per_minute: i32,

    #[validate(length(max = 50, message = "Category too long"))]
    pub category: Option<String>,

    /// Metabolic Equivalent of Task value
    #[validate(range(min = 1.0, max = 25.0, message = "MET value must be 1-25"))]
    pub met_value: Option<f64>,
}

/// Workout entry entity (actual exercise session)
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Workout {
    pub id: String,
    pub user_id: String,
    pub exercise_type_id: Option<String>,
    pub name: String,
    pub duration_minutes: i32,
    pub calories_burned: i32,
    pub intensity: String,
    pub performed_at: String,
    pub notes: Option<String>,
    pub heart_rate_avg: Option<i32>,
    pub created_at: String,
}

/// TAINT SOURCE: Create workout request from HTTP
/// Primary entry point for workout data
#[derive(Debug, Deserialize, Validate)]
pub struct CreateWorkoutRequest {
    /// Optional exercise type ID to use for calorie calculation
    pub exercise_type_id: Option<String>,

    #[validate(length(min = 1, max = 200, message = "Name must be 1-200 characters"))]
    pub name: String,

    #[validate(range(min = 1, max = 1440, message = "Duration must be 1-1440 minutes"))]
    pub duration_minutes: i32,

    /// Calories burned - calculated if exercise_type_id provided
    #[validate(range(min = 0, max = 50000, message = "Calories must be 0-50000"))]
    pub calories_burned: Option<i32>,

    /// Intensity: light, moderate, intense, extreme
    #[validate(length(max = 20, message = "Intensity too long"))]
    pub intensity: Option<String>,

    /// When the workout was performed (ISO 8601 datetime)
    #[validate(length(min = 1, message = "Performed at datetime required"))]
    pub performed_at: String,

    #[validate(length(max = 500, message = "Notes too long"))]
    pub notes: Option<String>,

    #[validate(range(min = 30, max = 250, message = "Heart rate must be 30-250 bpm"))]
    pub heart_rate_avg: Option<i32>,
}

/// TAINT SOURCE: Update workout request from HTTP
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateWorkoutRequest {
    #[validate(length(min = 1, max = 200))]
    pub name: Option<String>,

    #[validate(range(min = 1, max = 1440))]
    pub duration_minutes: Option<i32>,

    #[validate(range(min = 0, max = 50000))]
    pub calories_burned: Option<i32>,

    pub intensity: Option<String>,

    pub performed_at: Option<String>,

    #[validate(length(max = 500))]
    pub notes: Option<String>,

    #[validate(range(min = 30, max = 250))]
    pub heart_rate_avg: Option<i32>,
}

/// TAINT SOURCE: Search/filter workouts query params
#[derive(Debug, Deserialize)]
pub struct WorkoutSearchQuery {
    pub category: Option<String>,
    pub intensity: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub min_duration: Option<i32>,
    pub max_duration: Option<i32>,
    pub search: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

/// Workout with calculated statistics
#[derive(Debug, Serialize)]
pub struct WorkoutWithStats {
    #[serde(flatten)]
    pub workout: Workout,
    pub calories_per_minute: f64,
    pub intensity_multiplier: f64,
}

impl From<Workout> for WorkoutWithStats {
    fn from(workout: Workout) -> Self {
        let intensity = Intensity::from_str(&workout.intensity).unwrap_or(Intensity::Moderate);
        let calories_per_minute = if workout.duration_minutes > 0 {
            workout.calories_burned as f64 / workout.duration_minutes as f64
        } else {
            0.0
        };

        Self {
            workout,
            calories_per_minute,
            intensity_multiplier: intensity.calorie_multiplier(),
        }
    }
}
