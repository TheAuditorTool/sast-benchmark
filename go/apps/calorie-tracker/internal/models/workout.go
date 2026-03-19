package models

import (
	"time"

	"github.com/google/uuid"
	"gorm.io/gorm"
)

// WorkoutIntensity represents workout intensity level
type WorkoutIntensity string

const (
	IntensityLow      WorkoutIntensity = "low"
	IntensityModerate WorkoutIntensity = "moderate"
	IntensityHigh     WorkoutIntensity = "high"
	IntensityVeryHigh WorkoutIntensity = "very_high"
)

// Exercise represents an exercise type
type Exercise struct {
	ID                string  `gorm:"primaryKey" json:"id"`
	Name              string  `gorm:"uniqueIndex;not null" json:"name"`
	Category          string  `gorm:"not null" json:"category"`
	CaloriesPerMinute float64 `gorm:"not null" json:"calories_per_minute"`
	Description       string  `json:"description,omitempty"`
}

func (e *Exercise) BeforeCreate(tx *gorm.DB) error {
	if e.ID == "" {
		e.ID = uuid.New().String()
	}
	return nil
}

// Workout represents a workout entry
type Workout struct {
	ID              string           `gorm:"primaryKey" json:"id"`
	UserID          string           `gorm:"not null;index" json:"user_id"`
	ExerciseID      string           `gorm:"not null" json:"exercise_id"`
	ExerciseName    string           `gorm:"-" json:"exercise_name,omitempty"`
	DurationMinutes int              `gorm:"not null" json:"duration_minutes"`
	Intensity       WorkoutIntensity `gorm:"not null" json:"intensity"`
	CaloriesBurned  int              `gorm:"not null" json:"calories_burned"`
	Sets            *int             `json:"sets,omitempty"`
	Reps            *int             `json:"reps,omitempty"`
	WeightKg        *float64         `json:"weight_kg,omitempty"`
	Notes           string           `json:"notes,omitempty"`
	PerformedAt     time.Time        `gorm:"not null;index" json:"performed_at"`
	CreatedAt       time.Time        `json:"created_at"`
	UpdatedAt       time.Time        `json:"updated_at"`

	// Relationships
	User     *User     `gorm:"foreignKey:UserID" json:"user,omitempty"`
	Exercise *Exercise `gorm:"foreignKey:ExerciseID" json:"exercise,omitempty"`
}

func (w *Workout) BeforeCreate(tx *gorm.DB) error {
	if w.ID == "" {
		w.ID = uuid.New().String()
	}
	return nil
}

// CreateWorkoutRequest for creating a workout
// TAINT SOURCE: JSON body
type CreateWorkoutRequest struct {
	ExerciseID      string   `json:"exercise_id" validate:"required,uuid"`
	DurationMinutes int      `json:"duration_minutes" validate:"required,min=1,max=600"`
	Intensity       string   `json:"intensity" validate:"required,oneof=low moderate high very_high"`
	CaloriesBurned  int      `json:"calories_burned" validate:"required,min=1,max=5000"`
	Sets            *int     `json:"sets" validate:"omitempty,min=1,max=100"`
	Reps            *int     `json:"reps" validate:"omitempty,min=1,max=1000"`
	WeightKg        *float64 `json:"weight_kg" validate:"omitempty,min=0,max=1000"`
	Notes           string   `json:"notes" validate:"max=1000"`
	PerformedAt     string   `json:"performed_at" validate:"required"`
}

// UpdateWorkoutRequest for updating a workout
type UpdateWorkoutRequest struct {
	DurationMinutes *int     `json:"duration_minutes" validate:"omitempty,min=1,max=600"`
	Intensity       *string  `json:"intensity" validate:"omitempty,oneof=low moderate high very_high"`
	CaloriesBurned  *int     `json:"calories_burned" validate:"omitempty,min=1,max=5000"`
	Sets            *int     `json:"sets" validate:"omitempty,min=1,max=100"`
	Reps            *int     `json:"reps" validate:"omitempty,min=1,max=1000"`
	WeightKg        *float64 `json:"weight_kg" validate:"omitempty,min=0,max=1000"`
	Notes           *string  `json:"notes" validate:"omitempty,max=1000"`
}

// DailyWorkoutSummary aggregates workouts for a day
type DailyWorkoutSummary struct {
	Date                string    `json:"date"`
	TotalWorkouts       int       `json:"total_workouts"`
	TotalDuration       int       `json:"total_duration"`
	TotalCaloriesBurned int       `json:"total_calories_burned"`
	Workouts            []Workout `json:"workouts"`
}
