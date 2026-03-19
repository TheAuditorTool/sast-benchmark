//! Calorie Counter App - Main entry point.
//!
//! This is a full-featured calorie tracking web application demonstrating:
//! - Deep multi-hop taint analysis (HTTP -> Service -> Repository -> SQL)
//! - Cross-file data flow tracking
//! - Joi/Zod-style validation in Rust
//! - JWT authentication middleware
//!
//! Run with: cargo run
//! Access at: http://127.0.0.1:8080

use actix_files::Files;
use actix_web::{web, App, HttpServer, middleware::Logger};
use env_logger::Env;

use calorie_app::{
    config::AppConfig,
    db,
    handlers,
    middleware::AuthMiddleware,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Load configuration
    let config = AppConfig::from_env();
    let server_addr = config.server_addr();

    println!("========================================");
    println!("  Calorie Counter App");
    println!("========================================");
    println!("Environment: {}", config.environment);
    println!("Server:      http://{}", server_addr);
    println!("========================================");

    // Initialize database
    let pool = db::create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");

    // Run migrations
    db::run_migrations(&pool)
        .await
        .expect("Failed to run database migrations");

    println!("Database initialized successfully");
    println!("Starting server...\n");

    // Create shared state
    let pool_data = web::Data::new(pool);
    let config_data = web::Data::new(config.clone());

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // Add shared state
            .app_data(pool_data.clone())
            .app_data(config_data.clone())

            // Add logging middleware
            .wrap(Logger::default())

            // Static files (frontend)
            .service(Files::new("/static", "./static").index_file("index.html"))

            // Health check endpoint
            .route("/health", web::get().to(|| async {
                actix_web::HttpResponse::Ok().json(serde_json::json!({
                    "status": "healthy",
                    "service": "calorie-app"
                }))
            }))

            // Public auth routes (no auth required)
            .service(
                web::scope("/api/auth")
                    .route("/register", web::post().to(handlers::auth::register))
                    .route("/login", web::post().to(handlers::auth::login))
            )

            // Protected API routes (require auth)
            .service(
                web::scope("/api")
                    .wrap(AuthMiddleware::new(config.clone()))

                    // User profile routes
                    .route("/auth/me", web::get().to(handlers::auth::get_profile))
                    .route("/auth/profile", web::put().to(handlers::auth::update_profile))
                    .route("/auth/change-password", web::post().to(handlers::auth::change_password))
                    .route("/auth/account", web::delete().to(handlers::auth::delete_account))

                    // Food items routes
                    .route("/food-items", web::post().to(handlers::meals::create_food_item))
                    .route("/food-items", web::get().to(handlers::meals::list_food_items))
                    .route("/food-items/{id}", web::get().to(handlers::meals::get_food_item))

                    // Meals routes
                    .route("/meals", web::post().to(handlers::meals::create_meal))
                    .route("/meals", web::get().to(handlers::meals::search_meals))
                    .route("/meals/today", web::get().to(handlers::meals::get_todays_meals))
                    .route("/meals/date/{date}", web::get().to(handlers::meals::get_meals_by_date))
                    .route("/meals/{id}", web::get().to(handlers::meals::get_meal))
                    .route("/meals/{id}", web::put().to(handlers::meals::update_meal))
                    .route("/meals/{id}", web::delete().to(handlers::meals::delete_meal))

                    // Exercise types routes
                    .route("/exercise-types", web::post().to(handlers::workouts::create_exercise_type))
                    .route("/exercise-types", web::get().to(handlers::workouts::list_exercise_types))
                    .route("/exercise-types/{id}", web::get().to(handlers::workouts::get_exercise_type))

                    // Workouts routes
                    .route("/workouts", web::post().to(handlers::workouts::create_workout))
                    .route("/workouts", web::get().to(handlers::workouts::search_workouts))
                    .route("/workouts/today", web::get().to(handlers::workouts::get_todays_workouts))
                    .route("/workouts/date/{date}", web::get().to(handlers::workouts::get_workouts_by_date))
                    .route("/workouts/{id}", web::get().to(handlers::workouts::get_workout))
                    .route("/workouts/{id}", web::put().to(handlers::workouts::update_workout))
                    .route("/workouts/{id}", web::delete().to(handlers::workouts::delete_workout))

                    // Schedules routes
                    .route("/schedules", web::post().to(handlers::schedules::upsert_schedule))
                    .route("/schedules", web::get().to(handlers::schedules::get_weekly_schedule))
                    .route("/schedules/copy-week", web::post().to(handlers::schedules::copy_schedule_to_week))
                    .route("/schedules/{day}", web::get().to(handlers::schedules::get_schedule))
                    .route("/schedules/{day}", web::put().to(handlers::schedules::update_schedule))
                    .route("/schedules/{day}", web::delete().to(handlers::schedules::delete_schedule))

                    // Reports routes
                    .route("/reports/today", web::get().to(handlers::reports::get_today_progress))
                    .route("/reports/daily", web::get().to(handlers::reports::get_daily_report))
                    .route("/reports/weekly", web::get().to(handlers::reports::get_weekly_report))
                    .route("/reports/range", web::get().to(handlers::reports::get_range_report))
                    .route("/reports/nutrition", web::get().to(handlers::reports::get_nutrition_report))
                    .route("/reports/progress/{date}", web::get().to(handlers::reports::get_date_progress))
                    .route("/reports/goal-rate", web::get().to(handlers::reports::get_goal_rate))

                    // Dashboard (combined endpoint)
                    .route("/dashboard", web::get().to(handlers::reports::get_dashboard))
            )

            // Root redirect to static
            .route("/", web::get().to(|| async {
                actix_web::HttpResponse::Found()
                    .append_header(("Location", "/static/index.html"))
                    .finish()
            }))
    })
    .bind(&server_addr)?
    .run()
    .await
}
