package models

import (
	"time"

	"github.com/google/uuid"
	"gorm.io/gorm"
)

// DailyLog tracks daily metrics
type DailyLog struct {
	ID        string    `gorm:"primaryKey" json:"id"`
	UserID    string    `gorm:"not null;index" json:"user_id"`
	Date      string    `gorm:"not null;index" json:"date"` // YYYY-MM-DD format
	WaterMl   int       `gorm:"default:0" json:"water_ml"`
	Notes     string    `json:"notes,omitempty"`
	CreatedAt time.Time `json:"created_at"`
	UpdatedAt time.Time `json:"updated_at"`

	// Relationships
	User *User `gorm:"foreignKey:UserID" json:"user,omitempty"`
}

func (d *DailyLog) BeforeCreate(tx *gorm.DB) error {
	if d.ID == "" {
		d.ID = uuid.New().String()
	}
	return nil
}

// WeightLog tracks weight over time
type WeightLog struct {
	ID        string    `gorm:"primaryKey" json:"id"`
	UserID    string    `gorm:"not null;index" json:"user_id"`
	WeightKg  float64   `gorm:"not null" json:"weight_kg"`
	LoggedAt  time.Time `gorm:"not null;index" json:"logged_at"`
	Notes     string    `json:"notes,omitempty"`
	CreatedAt time.Time `json:"created_at"`

	// Relationships
	User *User `gorm:"foreignKey:UserID" json:"user,omitempty"`
}

func (w *WeightLog) BeforeCreate(tx *gorm.DB) error {
	if w.ID == "" {
		w.ID = uuid.New().String()
	}
	return nil
}

// WeightLogRequest for logging weight
type WeightLogRequest struct {
	WeightKg float64 `json:"weight_kg" validate:"required,min=20,max=500"`
	Notes    string  `json:"notes" validate:"max=500"`
}

// WaterLogRequest for logging water intake
type WaterLogRequest struct {
	AmountMl int `json:"amount_ml" validate:"required,min=1,max=10000"`
}

// DailySummary combines all daily data
type DailySummary struct {
	Date             string  `json:"date"`
	CaloriesConsumed int     `json:"calories_consumed"`
	CaloriesBurned   int     `json:"calories_burned"`
	NetCalories      int     `json:"net_calories"`
	CalorieGoal      int     `json:"calorie_goal"`
	ProteinGrams     float64 `json:"protein_grams"`
	CarbsGrams       float64 `json:"carbs_grams"`
	FatGrams         float64 `json:"fat_grams"`
	WaterMl          int     `json:"water_ml"`
	MealCount        int     `json:"meal_count"`
	WorkoutCount     int     `json:"workout_count"`
}

// WeeklySummary aggregates a week of data
type WeeklySummary struct {
	StartDate           string  `json:"start_date"`
	EndDate             string  `json:"end_date"`
	AvgCalories         float64 `json:"avg_calories"`
	TotalCaloriesBurned int     `json:"total_calories_burned"`
	TotalWorkouts       int     `json:"total_workouts"`
	AvgProtein          float64 `json:"avg_protein"`
	AvgCarbs            float64 `json:"avg_carbs"`
	AvgFat              float64 `json:"avg_fat"`
}

// ProgressData tracks progress over time
type ProgressData struct {
	Dates           []string  `json:"dates"`
	Calories        []int     `json:"calories"`
	CaloriesBurned  []int     `json:"calories_burned"`
	Weights         []float64 `json:"weights"`
	WeightDates     []string  `json:"weight_dates"`
	StartWeight     float64   `json:"start_weight"`
	CurrentWeight   float64   `json:"current_weight"`
	WeightChange    float64   `json:"weight_change"`
	AvgDailyCalories float64  `json:"avg_daily_calories"`
}
