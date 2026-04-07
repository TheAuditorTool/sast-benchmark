//! Schedule and goals models with validation.
//!
//! TAINT FLOW: HTTP request -> CreateScheduleRequest -> ScheduleService -> ScheduleRepository -> SQL

use serde::{Deserialize, Serialize};
use validator::Validate;

/// Days of the week
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum DayOfWeek {
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
}

impl DayOfWeek {
    pub fn from_i32(n: i32) -> Option<Self> {
        match n {
            0 => Some(DayOfWeek::Sunday),
            1 => Some(DayOfWeek::Monday),
            2 => Some(DayOfWeek::Tuesday),
            3 => Some(DayOfWeek::Wednesday),
            4 => Some(DayOfWeek::Thursday),
            5 => Some(DayOfWeek::Friday),
            6 => Some(DayOfWeek::Saturday),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            DayOfWeek::Sunday => "Sunday",
            DayOfWeek::Monday => "Monday",
            DayOfWeek::Tuesday => "Tuesday",
            DayOfWeek::Wednesday => "Wednesday",
            DayOfWeek::Thursday => "Thursday",
            DayOfWeek::Friday => "Friday",
            DayOfWeek::Saturday => "Saturday",
        }
    }
}

/// Schedule entry entity (daily goals)
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Schedule {
    pub id: String,
    pub user_id: String,
    pub day_of_week: i32,
    pub target_calories: i32,
    pub target_protein_grams: Option<f64>,
    pub target_carbs_grams: Option<f64>,
    pub target_fat_grams: Option<f64>,
    pub planned_workouts: Option<String>,
    pub notes: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// Schedule with day name for display
#[derive(Debug, Serialize)]
pub struct ScheduleWithDay {
    #[serde(flatten)]
    pub schedule: Schedule,
    pub day_name: String,
}

impl From<Schedule> for ScheduleWithDay {
    fn from(schedule: Schedule) -> Self {
        let day_name = DayOfWeek::from_i32(schedule.day_of_week)
            .map(|d| d.name())
            .unwrap_or("Unknown")
            .to_string();

        Self { schedule, day_name }
    }
}

/// TAINT SOURCE: Create schedule request from HTTP
#[derive(Debug, Deserialize, Validate)]
pub struct CreateScheduleRequest {
    #[validate(range(min = 0, max = 6, message = "Day of week must be 0-6 (Sun-Sat)"))]
    pub day_of_week: i32,

    #[validate(range(min = 500, max = 10000, message = "Target calories must be 500-10000"))]
    pub target_calories: i32,

    #[validate(range(min = 0.0, max = 500.0, message = "Protein must be 0-500g"))]
    pub target_protein_grams: Option<f64>,

    #[validate(range(min = 0.0, max = 1000.0, message = "Carbs must be 0-1000g"))]
    pub target_carbs_grams: Option<f64>,

    #[validate(range(min = 0.0, max = 500.0, message = "Fat must be 0-500g"))]
    pub target_fat_grams: Option<f64>,

    /// JSON string of planned workout descriptions
    #[validate(length(max = 2000, message = "Planned workouts too long"))]
    pub planned_workouts: Option<String>,

    #[validate(length(max = 500, message = "Notes too long"))]
    pub notes: Option<String>,
}

/// TAINT SOURCE: Update schedule request from HTTP
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateScheduleRequest {
    #[validate(range(min = 500, max = 10000))]
    pub target_calories: Option<i32>,

    #[validate(range(min = 0.0, max = 500.0))]
    pub target_protein_grams: Option<f64>,

    #[validate(range(min = 0.0, max = 1000.0))]
    pub target_carbs_grams: Option<f64>,

    #[validate(range(min = 0.0, max = 500.0))]
    pub target_fat_grams: Option<f64>,

    #[validate(length(max = 2000))]
    pub planned_workouts: Option<String>,

    #[validate(length(max = 500))]
    pub notes: Option<String>,

    pub is_active: Option<bool>,
}

/// Weekly schedule overview
#[derive(Debug, Serialize)]
pub struct WeeklySchedule {
    pub schedules: Vec<ScheduleWithDay>,
    pub total_weekly_calories: i32,
    pub avg_daily_calories: f64,
}

/// TAINT SOURCE: Quick log request - simplified meal/workout entry
#[derive(Debug, Deserialize, Validate)]
pub struct QuickLogRequest {
    /// Type: "meal" or "workout"
    #[validate(length(min = 1, message = "Log type required"))]
    pub log_type: String,

    #[validate(length(min = 1, max = 200, message = "Name required"))]
    pub name: String,

    /// Calories consumed or burned
    #[validate(range(min = 1, max = 50000, message = "Calories must be 1-50000"))]
    pub calories: i32,

    /// Duration in minutes (for workouts)
    #[validate(range(min = 1, max = 1440))]
    pub duration_minutes: Option<i32>,

    /// Meal type or exercise category
    pub category: Option<String>,

    #[validate(length(max = 500))]
    pub notes: Option<String>,
}

/// Goal progress tracking
#[derive(Debug, Serialize)]
pub struct GoalProgress {
    pub date: String,
    pub target_calories: i32,
    pub consumed_calories: i32,
    pub burned_calories: i32,
    pub net_calories: i32,
    pub remaining_calories: i32,
    pub progress_percent: f64,
    pub is_over_goal: bool,
}

impl GoalProgress {
    pub fn calculate(
        date: &str,
        target: i32,
        consumed: i32,
        burned: i32,
    ) -> Self {
        let net = consumed - burned;
        let remaining = target - net;
        let progress = if target > 0 {
            (net as f64 / target as f64) * 100.0
        } else {
            0.0
        };

        Self {
            date: date.to_string(),
            target_calories: target,
            consumed_calories: consumed,
            burned_calories: burned,
            net_calories: net,
            remaining_calories: remaining,
            progress_percent: progress.min(100.0),
            is_over_goal: net > target,
        }
    }
}
