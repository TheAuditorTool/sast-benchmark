// Calorie Tracker - Main Entry Point
//
// A full-featured calorie tracking web application built with Gin.
// Features:
// - User authentication with session management
// - CRUD operations for meals, workouts, and schedules
// - Daily calorie and macro tracking
// - Weight and water intake logging
// - Progress analytics and reporting

package main

import (
	"log"
	"os"

	"github.com/gin-contrib/sessions"
	"github.com/gin-contrib/sessions/cookie"
	"github.com/gin-gonic/gin"

	"github.com/example/calorie-tracker/internal/handlers"
	"github.com/example/calorie-tracker/internal/repository"
	"github.com/example/calorie-tracker/internal/services"
)

func main() {
	// Get configuration
	port := os.Getenv("PORT")
	if port == "" {
		port = "8080"
	}

	dbPath := os.Getenv("DATABASE_PATH")
	if dbPath == "" {
		dbPath = "calorie_tracker.db"
	}

	// Initialize database
	log.Println("Connecting to database:", dbPath)
	db, err := repository.NewDatabase(dbPath)
	if err != nil {
		log.Fatal("Failed to connect to database:", err)
	}

	// Run migrations
	log.Println("Running database migrations...")
	if err := db.AutoMigrate(); err != nil {
		log.Fatal("Failed to run migrations:", err)
	}

	// Seed initial data
	log.Println("Seeding initial data...")
	if err := db.SeedData(); err != nil {
		log.Println("Warning: Failed to seed data:", err)
	}

	// Create repositories
	userRepo := repository.NewUserRepository(db.DB)
	mealRepo := repository.NewMealRepository(db.DB)
	workoutRepo := repository.NewWorkoutRepository(db.DB)
	scheduleRepo := repository.NewScheduleRepository(db.DB)
	foodRepo := repository.NewFoodRepository(db.DB)
	analyticsRepo := repository.NewAnalyticsRepository(db.DB)

	// Create services
	userService := services.NewUserService(userRepo)
	mealService := services.NewMealService(mealRepo)
	workoutService := services.NewWorkoutService(workoutRepo)
	scheduleService := services.NewScheduleService(scheduleRepo)
	foodService := services.NewFoodService(foodRepo)
	analyticsService := services.NewAnalyticsService(analyticsRepo, mealRepo, workoutRepo, userRepo)

	// Create handlers
	h := &handlers.Handlers{
		Auth:      handlers.NewAuthHandler(userService),
		Meal:      handlers.NewMealHandler(mealService),
		Workout:   handlers.NewWorkoutHandler(workoutService),
		Schedule:  handlers.NewScheduleHandler(scheduleService),
		Dashboard: handlers.NewDashboardHandler(analyticsService),
		Food:      handlers.NewFoodHandler(foodService),
	}

	// Setup Gin
	r := gin.Default()

	// Session middleware
	store := cookie.NewStore([]byte("secret-key-change-in-production"))
	r.Use(sessions.Sessions("session", store))

	// Load templates
	r.LoadHTMLGlob("templates/*")

	// Static files
	r.Static("/static", "./static")

	// Setup routes
	handlers.SetupRoutes(r, h)

	// Start server
	log.Printf("Starting server at http://localhost:%s", port)
	if err := r.Run(":" + port); err != nil {
		log.Fatal("Failed to start server:", err)
	}
}
