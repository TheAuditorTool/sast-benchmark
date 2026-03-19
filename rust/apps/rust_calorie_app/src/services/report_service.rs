//! Report service - Analytics and summary generation.
//!
//! TAINT TRANSFORM: Aggregates data from multiple sources.

use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::{DailySummary, GoalProgress, NutritionInfo, Schedule};
use crate::repositories::{MealRepository, ScheduleRepository, WorkoutRepository};
use crate::services::CalorieCalculator;
use chrono::{Datelike, NaiveDate, Utc};

/// Service for reports and analytics
pub struct ReportService;

impl ReportService {
    /// Get daily progress summary.
    ///
    /// TAINT TRANSFORM: Aggregates meal and workout data for a date.
    /// Cross-file flow: date param -> meal queries + workout queries -> aggregated result
    pub async fn get_daily_progress(
        pool: &DbPool,
        user_id: &str,
        date: &str,
    ) -> Result<GoalProgress, AppError> {
        // Get daily goal from schedule
        let day_of_week = Self::date_to_day_of_week(date)?;
        let schedule = ScheduleRepository::find_by_day(pool, user_id, day_of_week).await?;
        let target = schedule.map(|s| s.target_calories).unwrap_or(2000);

        // TAINT FLOW: date -> MealRepository -> SQL aggregation
        let (consumed, _protein, _carbs, _fat, _meal_count) =
            MealRepository::get_daily_totals(pool, user_id, date).await?;

        // TAINT FLOW: date -> WorkoutRepository -> SQL aggregation
        let (burned, _duration, _workout_count) =
            WorkoutRepository::get_daily_totals(pool, user_id, date).await?;

        Ok(GoalProgress::calculate(date, target, consumed, burned))
    }

    /// Get detailed daily summary with nutrition breakdown.
    pub async fn get_daily_summary(
        pool: &DbPool,
        user_id: &str,
        date: &str,
    ) -> Result<DailySummary, AppError> {
        // Check cache first
        if let Some(summary) = ScheduleRepository::get_daily_summary(pool, user_id, date).await? {
            return Ok(summary);
        }

        // Calculate from raw data
        let (consumed, protein, carbs, fat, meal_count) =
            MealRepository::get_daily_totals(pool, user_id, date).await?;

        let (burned, _duration, workout_count) =
            WorkoutRepository::get_daily_totals(pool, user_id, date).await?;

        let day_of_week = Self::date_to_day_of_week(date)?;
        let schedule = ScheduleRepository::find_by_day(pool, user_id, day_of_week).await?;
        let target = schedule.map(|s| s.target_calories).unwrap_or(2000);

        let net = consumed - burned;
        let goal_met = net <= target;

        // Cache the result
        ScheduleRepository::upsert_daily_summary(
            pool,
            user_id,
            date,
            consumed,
            burned,
            protein,
            carbs,
            fat,
            meal_count,
            workout_count,
            target,
        )
        .await?;

        Ok(DailySummary {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            date: date.to_string(),
            total_calories_consumed: consumed,
            total_calories_burned: burned,
            net_calories: net,
            total_protein_grams: protein,
            total_carbs_grams: carbs,
            total_fat_grams: fat,
            meal_count,
            workout_count,
            goal_met,
        })
    }

    /// Get weekly summary.
    ///
    /// TAINT TRANSFORM: date range -> daily summaries -> weekly totals
    pub async fn get_weekly_summary(
        pool: &DbPool,
        user_id: &str,
        week_start: &str,
    ) -> Result<Vec<DailySummary>, AppError> {
        // Calculate end of week
        let start = NaiveDate::parse_from_str(week_start, "%Y-%m-%d")
            .map_err(|_| AppError::Validation("Invalid date format".to_string()))?;
        let end = start + chrono::Duration::days(6);
        let end_str = end.format("%Y-%m-%d").to_string();

        // TAINT FLOW: date range -> ScheduleRepository -> SQL query
        let summaries =
            ScheduleRepository::get_summaries_range(pool, user_id, week_start, &end_str).await?;

        // Fill in missing days
        let mut result = Vec::new();
        let mut current = start;
        while current <= end {
            let date_str = current.format("%Y-%m-%d").to_string();

            if let Some(summary) = summaries.iter().find(|s| s.date == date_str) {
                result.push(summary.clone());
            } else {
                // Generate empty summary for missing day
                result.push(DailySummary {
                    id: uuid::Uuid::new_v4().to_string(),
                    user_id: user_id.to_string(),
                    date: date_str,
                    total_calories_consumed: 0,
                    total_calories_burned: 0,
                    net_calories: 0,
                    total_protein_grams: 0.0,
                    total_carbs_grams: 0.0,
                    total_fat_grams: 0.0,
                    meal_count: 0,
                    workout_count: 0,
                    goal_met: true, // Empty day meets goal
                });
            }

            current = current.succ_opt().unwrap_or(current);
        }

        Ok(result)
    }

    /// Get nutrition averages for date range.
    pub async fn get_nutrition_averages(
        pool: &DbPool,
        user_id: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<NutritionInfo, AppError> {
        // TAINT FLOW: date range -> repository -> aggregation
        let summaries =
            ScheduleRepository::get_summaries_range(pool, user_id, start_date, end_date).await?;

        if summaries.is_empty() {
            return Ok(NutritionInfo::default());
        }

        let count = summaries.len() as f64;
        let total_cal: i32 = summaries.iter().map(|s| s.total_calories_consumed).sum();
        let total_protein: f64 = summaries.iter().map(|s| s.total_protein_grams).sum();
        let total_carbs: f64 = summaries.iter().map(|s| s.total_carbs_grams).sum();
        let total_fat: f64 = summaries.iter().map(|s| s.total_fat_grams).sum();

        Ok(NutritionInfo {
            calories: (total_cal as f64 / count).round() as i32,
            protein_grams: total_protein / count,
            carbs_grams: total_carbs / count,
            fat_grams: total_fat / count,
        })
    }

    /// Get goal achievement rate for date range.
    pub async fn get_goal_achievement_rate(
        pool: &DbPool,
        user_id: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<f64, AppError> {
        let summaries =
            ScheduleRepository::get_summaries_range(pool, user_id, start_date, end_date).await?;

        if summaries.is_empty() {
            return Ok(0.0);
        }

        let met_count = summaries.iter().filter(|s| s.goal_met).count();
        Ok((met_count as f64 / summaries.len() as f64) * 100.0)
    }

    /// Refresh daily summary (called after meal/workout changes).
    pub async fn refresh_daily_summary(
        pool: &DbPool,
        user_id: &str,
        date: &str,
    ) -> Result<(), AppError> {
        // Force recalculation
        let (consumed, protein, carbs, fat, meal_count) =
            MealRepository::get_daily_totals(pool, user_id, date).await?;

        let (burned, _duration, workout_count) =
            WorkoutRepository::get_daily_totals(pool, user_id, date).await?;

        let day_of_week = Self::date_to_day_of_week(date)?;
        let schedule = ScheduleRepository::find_by_day(pool, user_id, day_of_week).await?;
        let target = schedule.map(|s| s.target_calories).unwrap_or(2000);

        ScheduleRepository::upsert_daily_summary(
            pool, user_id, date, consumed, burned, protein, carbs, fat, meal_count, workout_count,
            target,
        )
        .await
    }

    // ==================== Private Helpers ====================

    fn date_to_day_of_week(date: &str) -> Result<i32, AppError> {
        let parsed = NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .map_err(|_| AppError::Validation("Invalid date format".to_string()))?;

        // weekday() returns Monday=0 through Sunday=6
        // We want Sunday=0 through Saturday=6
        let day = parsed.weekday().num_days_from_sunday() as i32;
        Ok(day)
    }

    /// Get today's date string.
    pub fn today() -> String {
        Utc::now().format("%Y-%m-%d").to_string()
    }
}
