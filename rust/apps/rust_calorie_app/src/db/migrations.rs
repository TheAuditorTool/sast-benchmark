//! Database migrations management.
//!
//! Runs SQL migrations to set up the schema.

use sqlx::SqlitePool;

/// Run all database migrations.
///
/// This executes the initial schema creation.
pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("Running database migrations...");

    // Create tables using raw SQL
    // In production, use sqlx::migrate!() macro

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            email TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            username TEXT NOT NULL,
            height_cm REAL,
            weight_kg REAL,
            age INTEGER,
            daily_calorie_goal INTEGER DEFAULT 2000,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS food_items (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            name TEXT NOT NULL,
            calories_per_serving INTEGER NOT NULL,
            protein_grams REAL DEFAULT 0,
            carbs_grams REAL DEFAULT 0,
            fat_grams REAL DEFAULT 0,
            serving_size TEXT DEFAULT '1 serving',
            category TEXT DEFAULT 'other',
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS meals (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            food_item_id TEXT,
            name TEXT NOT NULL,
            calories INTEGER NOT NULL,
            protein_grams REAL DEFAULT 0,
            carbs_grams REAL DEFAULT 0,
            fat_grams REAL DEFAULT 0,
            servings REAL DEFAULT 1.0,
            meal_type TEXT NOT NULL,
            consumed_at TEXT NOT NULL,
            notes TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY (food_item_id) REFERENCES food_items(id) ON DELETE SET NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS exercise_types (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            name TEXT NOT NULL,
            calories_per_minute INTEGER NOT NULL,
            category TEXT DEFAULT 'cardio',
            met_value REAL DEFAULT 5.0,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS workouts (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            exercise_type_id TEXT,
            name TEXT NOT NULL,
            duration_minutes INTEGER NOT NULL,
            calories_burned INTEGER NOT NULL,
            intensity TEXT DEFAULT 'moderate',
            performed_at TEXT NOT NULL,
            notes TEXT,
            heart_rate_avg INTEGER,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY (exercise_type_id) REFERENCES exercise_types(id) ON DELETE SET NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS schedules (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            day_of_week INTEGER NOT NULL,
            target_calories INTEGER NOT NULL,
            target_protein_grams REAL,
            target_carbs_grams REAL,
            target_fat_grams REAL,
            planned_workouts TEXT,
            notes TEXT,
            is_active INTEGER DEFAULT 1,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
            UNIQUE(user_id, day_of_week)
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS daily_summaries (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            date TEXT NOT NULL,
            total_calories_consumed INTEGER DEFAULT 0,
            total_calories_burned INTEGER DEFAULT 0,
            net_calories INTEGER DEFAULT 0,
            total_protein_grams REAL DEFAULT 0,
            total_carbs_grams REAL DEFAULT 0,
            total_fat_grams REAL DEFAULT 0,
            meal_count INTEGER DEFAULT 0,
            workout_count INTEGER DEFAULT 0,
            goal_met INTEGER DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
            UNIQUE(user_id, date)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create indexes for performance
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_meals_user_date ON meals(user_id, consumed_at)")
        .execute(pool)
        .await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_workouts_user_date ON workouts(user_id, performed_at)",
    )
    .execute(pool)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_food_items_user ON food_items(user_id)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_schedules_user ON schedules(user_id, day_of_week)")
        .execute(pool)
        .await?;

    println!("Migrations completed successfully");

    Ok(())
}
