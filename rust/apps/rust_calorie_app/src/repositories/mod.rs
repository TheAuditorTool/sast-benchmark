//! Repository layer - Database access with SQL queries.
//!
//! This layer contains all SQL queries (TAINT SINKS).
//! Data flows: handlers -> services -> repositories -> database
//!
//! TAINT SINK: All repository methods execute SQL with data from upstream.

pub mod meal_repo;
pub mod schedule_repo;
pub mod user_repo;
pub mod workout_repo;

pub use meal_repo::MealRepository;
pub use schedule_repo::ScheduleRepository;
pub use user_repo::UserRepository;
pub use workout_repo::WorkoutRepository;
