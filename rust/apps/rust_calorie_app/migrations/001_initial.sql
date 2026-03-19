-- Initial schema for calorie tracking app
-- Run with: sqlx database create && sqlx migrate run

-- Users table
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
);

-- Food items catalog (reusable foods)
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
);

-- Meal entries (actual consumption log)
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
    meal_type TEXT NOT NULL CHECK (meal_type IN ('breakfast', 'lunch', 'dinner', 'snack')),
    consumed_at TEXT NOT NULL,
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (food_item_id) REFERENCES food_items(id) ON DELETE SET NULL
);

-- Exercise types catalog
CREATE TABLE IF NOT EXISTS exercise_types (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    calories_per_minute INTEGER NOT NULL,
    category TEXT DEFAULT 'cardio' CHECK (category IN ('cardio', 'strength', 'flexibility', 'sports', 'other')),
    met_value REAL DEFAULT 5.0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Workout entries (actual exercise log)
CREATE TABLE IF NOT EXISTS workouts (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    exercise_type_id TEXT,
    name TEXT NOT NULL,
    duration_minutes INTEGER NOT NULL,
    calories_burned INTEGER NOT NULL,
    intensity TEXT DEFAULT 'moderate' CHECK (intensity IN ('light', 'moderate', 'intense', 'extreme')),
    performed_at TEXT NOT NULL,
    notes TEXT,
    heart_rate_avg INTEGER,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (exercise_type_id) REFERENCES exercise_types(id) ON DELETE SET NULL
);

-- Daily schedules/goals
CREATE TABLE IF NOT EXISTS schedules (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    day_of_week INTEGER NOT NULL CHECK (day_of_week BETWEEN 0 AND 6),
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
);

-- Daily summaries (cached aggregations)
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
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_meals_user_date ON meals(user_id, consumed_at);
CREATE INDEX IF NOT EXISTS idx_workouts_user_date ON workouts(user_id, performed_at);
CREATE INDEX IF NOT EXISTS idx_schedules_user ON schedules(user_id, day_of_week);
CREATE INDEX IF NOT EXISTS idx_daily_summaries_user_date ON daily_summaries(user_id, date);
CREATE INDEX IF NOT EXISTS idx_food_items_user ON food_items(user_id);
CREATE INDEX IF NOT EXISTS idx_exercise_types_user ON exercise_types(user_id);
