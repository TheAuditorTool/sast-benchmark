//! Schedule repository - SQL queries for schedules and daily goals.
//!
//! TAINT SINK: Schedule data flows from HTTP to database.

use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::{CreateScheduleRequest, DailySummary, Schedule, UpdateScheduleRequest};
use sqlx::Row;
use uuid::Uuid;

/// Repository for schedule and summary database operations
pub struct ScheduleRepository;

impl ScheduleRepository {
    // vuln-code-snippet start sqliCalorieUpsertSchedule
    /// Create or update a schedule for a day of week.
    ///
    /// TAINT SINK: CreateScheduleRequest fields flow to UPSERT
    pub async fn upsert_schedule(
        pool: &DbPool,
        user_id: &str,
        request: &CreateScheduleRequest,
    ) -> Result<Schedule, AppError> {
        let id = Uuid::new_v4().to_string();

        // TAINT SINK: All request fields to SQL
        sqlx::query(
            r#"
            INSERT INTO schedules (
                id, user_id, day_of_week, target_calories, target_protein_grams,
                target_carbs_grams, target_fat_grams, planned_workouts, notes,
                is_active, created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 1, datetime('now'), datetime('now'))
            ON CONFLICT(user_id, day_of_week) DO UPDATE SET
                target_calories = excluded.target_calories,
                target_protein_grams = excluded.target_protein_grams,
                target_carbs_grams = excluded.target_carbs_grams,
                target_fat_grams = excluded.target_fat_grams,
                planned_workouts = excluded.planned_workouts,
                notes = excluded.notes,
                updated_at = datetime('now')
            "#,
        )
        // vuln-code-snippet target-line sqliCalorieUpsertSchedule
        .bind(&id)
        .bind(user_id)
        .bind(request.day_of_week)                   // TAINT: from HTTP
        .bind(request.target_calories)               // TAINT: from HTTP
        .bind(request.target_protein_grams)          // TAINT: from HTTP
        .bind(request.target_carbs_grams)            // TAINT: from HTTP
        .bind(request.target_fat_grams)              // TAINT: from HTTP
        .bind(&request.planned_workouts)             // TAINT: from HTTP
        .bind(&request.notes)                        // TAINT: from HTTP
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Self::find_by_day(pool, user_id, request.day_of_week)
            .await?
            .ok_or(AppError::NotFound("Schedule not found".to_string()))
    }
    // vuln-code-snippet end sqliCalorieUpsertSchedule

    /// Find schedule by day of week.
    pub async fn find_by_day(
        pool: &DbPool,
        user_id: &str,
        day_of_week: i32,
    ) -> Result<Option<Schedule>, AppError> {
        // TAINT SINK: day_of_week in WHERE
        let schedule = sqlx::query_as::<_, Schedule>(
            "SELECT * FROM schedules WHERE user_id = ? AND day_of_week = ? AND is_active = 1",
        )
        .bind(user_id)
        .bind(day_of_week)  // TAINT: from path/query param
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(schedule)
    }

    /// Get all schedules for a user (weekly view).
    pub async fn get_weekly_schedule(
        pool: &DbPool,
        user_id: &str,
    ) -> Result<Vec<Schedule>, AppError> {
        let schedules = sqlx::query_as::<_, Schedule>(
            "SELECT * FROM schedules WHERE user_id = ? AND is_active = 1 ORDER BY day_of_week",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(schedules)
    }

    // vuln-code-snippet start sqliCalorieUpdateSchedule
    /// Update a schedule.
    ///
    /// TAINT SINK: UpdateScheduleRequest fields flow to UPDATE
    pub async fn update_schedule(
        pool: &DbPool,
        user_id: &str,
        day_of_week: i32,
        request: &UpdateScheduleRequest,
    ) -> Result<Schedule, AppError> {
        let mut updates = Vec::new();

        // Build dynamic update - TAINT through all fields
        if let Some(cal) = request.target_calories {
            updates.push(format!("target_calories = {}", cal));
        }
        if let Some(protein) = request.target_protein_grams {
            updates.push(format!("target_protein_grams = {}", protein));
        }
        if let Some(carbs) = request.target_carbs_grams {
            updates.push(format!("target_carbs_grams = {}", carbs));
        }
        if let Some(fat) = request.target_fat_grams {
            updates.push(format!("target_fat_grams = {}", fat));
        }
        if let Some(ref workouts) = request.planned_workouts {
            // vuln-code-snippet target-line sqliCalorieUpdateSchedule
            updates.push(format!("planned_workouts = '{}'", workouts.replace("'", "''")));
        }
        if let Some(ref notes) = request.notes {
            updates.push(format!("notes = '{}'", notes.replace("'", "''")));
        }
        if let Some(active) = request.is_active {
            updates.push(format!("is_active = {}", if active { 1 } else { 0 }));
        }

        if updates.is_empty() {
            return Self::find_by_day(pool, user_id, day_of_week)
                .await?
                .ok_or(AppError::NotFound("Schedule not found".to_string()));
        }

        updates.push("updated_at = datetime('now')".to_string());

        // TAINT SINK: Dynamic UPDATE
        let query = format!(
            "UPDATE schedules SET {} WHERE user_id = ? AND day_of_week = ?",
            updates.join(", ")
        );

        sqlx::query(&query)
            .bind(user_id)
            .bind(day_of_week)
            .execute(pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Self::find_by_day(pool, user_id, day_of_week)
            .await?
            .ok_or(AppError::NotFound("Schedule not found".to_string()))
    }
    // vuln-code-snippet end sqliCalorieUpdateSchedule

    /// Delete a schedule.
    pub async fn delete_schedule(
        pool: &DbPool,
        user_id: &str,
        day_of_week: i32,
    ) -> Result<bool, AppError> {
        let result = sqlx::query(
            "DELETE FROM schedules WHERE user_id = ? AND day_of_week = ?",
        )
        .bind(user_id)
        .bind(day_of_week)  // TAINT: from path param
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    // ==================== Daily Summaries ====================

    /// Update or create daily summary.
    ///
    /// Called after meal/workout changes to update cached totals.
    pub async fn upsert_daily_summary(
        pool: &DbPool,
        user_id: &str,
        date: &str,
        consumed: i32,
        burned: i32,
        protein: f64,
        carbs: f64,
        fat: f64,
        meal_count: i32,
        workout_count: i32,
        goal: i32,
    ) -> Result<(), AppError> {
        let id = Uuid::new_v4().to_string();
        let net = consumed - burned;
        let goal_met = net <= goal;

        // TAINT SINK: All calculated values to SQL
        sqlx::query(
            r#"
            INSERT INTO daily_summaries (
                id, user_id, date, total_calories_consumed, total_calories_burned,
                net_calories, total_protein_grams, total_carbs_grams, total_fat_grams,
                meal_count, workout_count, goal_met, created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))
            ON CONFLICT(user_id, date) DO UPDATE SET
                total_calories_consumed = excluded.total_calories_consumed,
                total_calories_burned = excluded.total_calories_burned,
                net_calories = excluded.net_calories,
                total_protein_grams = excluded.total_protein_grams,
                total_carbs_grams = excluded.total_carbs_grams,
                total_fat_grams = excluded.total_fat_grams,
                meal_count = excluded.meal_count,
                workout_count = excluded.workout_count,
                goal_met = excluded.goal_met,
                updated_at = datetime('now')
            "#,
        )
        .bind(&id)
        .bind(user_id)
        .bind(date)              // TAINT: from calculation
        .bind(consumed)          // TAINT: aggregated from meals
        .bind(burned)            // TAINT: aggregated from workouts
        .bind(net)
        .bind(protein)           // TAINT: aggregated
        .bind(carbs)             // TAINT: aggregated
        .bind(fat)               // TAINT: aggregated
        .bind(meal_count)
        .bind(workout_count)
        .bind(goal_met as i32)
        .execute(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    /// Get daily summary for a date.
    pub async fn get_daily_summary(
        pool: &DbPool,
        user_id: &str,
        date: &str,
    ) -> Result<Option<DailySummary>, AppError> {
        // TAINT SINK: date in WHERE
        let row = sqlx::query(
            "SELECT * FROM daily_summaries WHERE user_id = ? AND date = ?",
        )
        .bind(user_id)
        .bind(date)  // TAINT: from query param
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(row.map(|r| DailySummary {
            id: r.get("id"),
            user_id: r.get("user_id"),
            date: r.get("date"),
            total_calories_consumed: r.get("total_calories_consumed"),
            total_calories_burned: r.get("total_calories_burned"),
            net_calories: r.get("net_calories"),
            total_protein_grams: r.get("total_protein_grams"),
            total_carbs_grams: r.get("total_carbs_grams"),
            total_fat_grams: r.get("total_fat_grams"),
            meal_count: r.get("meal_count"),
            workout_count: r.get("workout_count"),
            goal_met: r.get::<i32, _>("goal_met") != 0,
        }))
    }

    /// Get summaries for date range.
    ///
    /// TAINT SINK: start_date, end_date flow to WHERE clause
    pub async fn get_summaries_range(
        pool: &DbPool,
        user_id: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<DailySummary>, AppError> {
        // TAINT SINK: Date range params in SQL
        let rows = sqlx::query(
            r#"
            SELECT * FROM daily_summaries
            WHERE user_id = ? AND date >= ? AND date <= ?
            ORDER BY date DESC
            "#,
        )
        .bind(user_id)
        .bind(start_date)  // TAINT: from query param
        .bind(end_date)    // TAINT: from query param
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        let summaries: Vec<DailySummary> = rows
            .iter()
            .map(|r| DailySummary {
                id: r.get("id"),
                user_id: r.get("user_id"),
                date: r.get("date"),
                total_calories_consumed: r.get("total_calories_consumed"),
                total_calories_burned: r.get("total_calories_burned"),
                net_calories: r.get("net_calories"),
                total_protein_grams: r.get("total_protein_grams"),
                total_carbs_grams: r.get("total_carbs_grams"),
                total_fat_grams: r.get("total_fat_grams"),
                meal_count: r.get("meal_count"),
                workout_count: r.get("workout_count"),
                goal_met: r.get::<i32, _>("goal_met") != 0,
            })
            .collect();

        Ok(summaries)
    }
}
