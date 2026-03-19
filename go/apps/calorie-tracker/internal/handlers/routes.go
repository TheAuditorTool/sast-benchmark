package handlers

import (
	"github.com/gin-gonic/gin"

	"github.com/example/calorie-tracker/internal/middleware"
)

// Handlers holds all handler instances
type Handlers struct {
	Auth      *AuthHandler
	Meal      *MealHandler
	Workout   *WorkoutHandler
	Schedule  *ScheduleHandler
	Dashboard *DashboardHandler
	Food      *FoodHandler
}

// SetupRoutes configures all routes for the application
func SetupRoutes(r *gin.Engine, h *Handlers) {
	// Auth routes (no auth required)
	auth := r.Group("/api/auth")
	{
		auth.POST("/register", h.Auth.Register)
		auth.POST("/login", h.Auth.Login)
		auth.POST("/logout", h.Auth.Logout)
	}

	// Auth routes requiring authentication
	authProtected := r.Group("/api/auth")
	authProtected.Use(middleware.AuthRequired())
	{
		authProtected.GET("/profile", h.Auth.GetProfile)
		authProtected.PUT("/profile", h.Auth.UpdateProfile)
	}

	// Meal routes (auth required)
	meals := r.Group("/api/meals")
	meals.Use(middleware.AuthRequired())
	{
		meals.GET("", h.Meal.ListMeals)
		meals.POST("", h.Meal.CreateMeal)
		meals.POST("/quick", h.Meal.QuickLog)
		meals.GET("/search", h.Meal.SearchMeals)
		meals.GET("/daily/:date", h.Meal.GetDailyMeals)
		meals.GET("/:id", h.Meal.GetMeal)
		meals.PUT("/:id", h.Meal.UpdateMeal)
		meals.DELETE("/:id", h.Meal.DeleteMeal)
	}

	// Workout routes (auth required)
	workouts := r.Group("/api/workouts")
	workouts.Use(middleware.AuthRequired())
	{
		workouts.GET("", h.Workout.ListWorkouts)
		workouts.POST("", h.Workout.CreateWorkout)
		workouts.GET("/daily/:date", h.Workout.GetDailyWorkouts)
		workouts.GET("/:id", h.Workout.GetWorkout)
		workouts.PUT("/:id", h.Workout.UpdateWorkout)
		workouts.DELETE("/:id", h.Workout.DeleteWorkout)
	}

	// Exercise routes (auth required)
	r.GET("/api/exercises", middleware.AuthRequired(), h.Workout.ListExercises)

	// Schedule routes (auth required)
	schedules := r.Group("/api/schedules")
	schedules.Use(middleware.AuthRequired())
	{
		schedules.GET("", h.Schedule.ListSchedules)
		schedules.POST("", h.Schedule.CreateSchedule)
		schedules.GET("/:id", h.Schedule.GetSchedule)
		schedules.PUT("/:id", h.Schedule.UpdateSchedule)
		schedules.DELETE("/:id", h.Schedule.DeleteSchedule)
		schedules.POST("/:id/toggle", h.Schedule.ToggleSchedule)
		schedules.POST("/:id/items", h.Schedule.AddItem)
		schedules.DELETE("/items/:item_id", h.Schedule.DeleteItem)
	}

	// Food routes (no auth required for read, auth for create)
	foods := r.Group("/api/foods")
	{
		foods.GET("", h.Food.ListFoods)
		foods.GET("/search", h.Food.SearchFoods)
		foods.GET("/:id", h.Food.GetFood)
		foods.POST("", middleware.AuthRequired(), h.Food.CreateFood)
	}

	// Dashboard routes (auth required)
	dashboard := r.Group("/api/dashboard")
	dashboard.Use(middleware.AuthRequired())
	{
		dashboard.GET("/today", h.Dashboard.GetToday)
		dashboard.GET("/summary/:date", h.Dashboard.GetDailySummary)
		dashboard.GET("/weekly", h.Dashboard.GetWeeklySummary)
		dashboard.GET("/progress", h.Dashboard.GetProgress)
		dashboard.POST("/weight", h.Dashboard.LogWeight)
		dashboard.POST("/water", h.Dashboard.LogWater)
	}

	// Admin routes (vulnerable - auth required)
	admin := r.Group("/api/admin")
	admin.Use(middleware.AuthRequired())
	{
		admin.POST("/query", h.Dashboard.RawQuery)
		admin.POST("/report", h.Dashboard.CustomReport)
	}

	// Frontend pages
	r.GET("/", middleware.OptionalAuth(), h.Dashboard.IndexPage)
	r.GET("/login", h.Auth.LoginPage)
	r.GET("/register", h.Auth.RegisterPage)
	r.GET("/dashboard", middleware.AuthRequired(), h.Dashboard.DashboardPage)
	r.GET("/meals", middleware.AuthRequired(), h.Meal.MealsPage)
	r.GET("/workouts", middleware.AuthRequired(), h.Workout.WorkoutsPage)
	r.GET("/schedule", middleware.AuthRequired(), h.Schedule.SchedulePage)
}
