//! Service layer - Business logic and data transformation.
//!
//! Services sit between handlers (HTTP) and repositories (database).
//! They validate, transform, and orchestrate data flow.
//!
//! TAINT TRANSFORM: Data flows through services from HTTP sources to SQL sinks.
//! Cross-file flow: handlers -> services -> repositories

pub mod auth_service;
pub mod calorie_calculator;
pub mod meal_service;
pub mod report_service;
pub mod workout_service;

pub use auth_service::AuthService;
pub use calorie_calculator::CalorieCalculator;
pub use meal_service::MealService;
pub use report_service::ReportService;
pub use workout_service::WorkoutService;
