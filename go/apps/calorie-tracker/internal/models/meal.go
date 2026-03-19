package models

import (
	"time"

	"github.com/google/uuid"
	"gorm.io/gorm"
)

// MealType represents the type of meal
type MealType string

const (
	MealTypeBreakfast MealType = "breakfast"
	MealTypeLunch     MealType = "lunch"
	MealTypeDinner    MealType = "dinner"
	MealTypeSnack     MealType = "snack"
)

// Meal represents a meal entry
type Meal struct {
	ID           string    `gorm:"primaryKey" json:"id"`
	UserID       string    `gorm:"not null;index" json:"user_id"`
	Name         string    `gorm:"not null" json:"name"`
	Description  string    `json:"description,omitempty"`
	MealType     MealType  `gorm:"not null" json:"meal_type"`
	Calories     int       `gorm:"not null" json:"calories"`
	ProteinGrams *float64  `json:"protein_grams,omitempty"`
	CarbsGrams   *float64  `json:"carbs_grams,omitempty"`
	FatGrams     *float64  `json:"fat_grams,omitempty"`
	ConsumedAt   time.Time `gorm:"not null;index" json:"consumed_at"`
	CreatedAt    time.Time `json:"created_at"`
	UpdatedAt    time.Time `json:"updated_at"`

	// Relationships
	User *User `gorm:"foreignKey:UserID" json:"user,omitempty"`
}

func (m *Meal) BeforeCreate(tx *gorm.DB) error {
	if m.ID == "" {
		m.ID = uuid.New().String()
	}
	return nil
}

// CreateMealRequest for creating a meal
// TAINT SOURCE: JSON body flows to database
type CreateMealRequest struct {
	Name         string   `json:"name" validate:"required,min=1,max=200"`
	Description  string   `json:"description" validate:"max=1000"`
	MealType     string   `json:"meal_type" validate:"required,oneof=breakfast lunch dinner snack"`
	Calories     int      `json:"calories" validate:"required,min=1,max=10000"`
	ProteinGrams *float64 `json:"protein_grams" validate:"omitempty,min=0,max=500"`
	CarbsGrams   *float64 `json:"carbs_grams" validate:"omitempty,min=0,max=1000"`
	FatGrams     *float64 `json:"fat_grams" validate:"omitempty,min=0,max=500"`
	ConsumedAt   string   `json:"consumed_at" validate:"required"`
}

// UpdateMealRequest for updating a meal
type UpdateMealRequest struct {
	Name         *string  `json:"name" validate:"omitempty,min=1,max=200"`
	Description  *string  `json:"description" validate:"omitempty,max=1000"`
	MealType     *string  `json:"meal_type" validate:"omitempty,oneof=breakfast lunch dinner snack"`
	Calories     *int     `json:"calories" validate:"omitempty,min=1,max=10000"`
	ProteinGrams *float64 `json:"protein_grams" validate:"omitempty,min=0,max=500"`
	CarbsGrams   *float64 `json:"carbs_grams" validate:"omitempty,min=0,max=1000"`
	FatGrams     *float64 `json:"fat_grams" validate:"omitempty,min=0,max=500"`
}

// QuickMealEntry for quick meal logging
type QuickMealEntry struct {
	Name     string `json:"name" validate:"required,min=1,max=200"`
	Calories int    `json:"calories" validate:"required,min=1,max=10000"`
	MealType string `json:"meal_type" validate:"omitempty,oneof=breakfast lunch dinner snack"`
}

// DailyMealSummary aggregates meals for a day
type DailyMealSummary struct {
	Date          string  `json:"date"`
	TotalCalories int     `json:"total_calories"`
	TotalProtein  float64 `json:"total_protein"`
	TotalCarbs    float64 `json:"total_carbs"`
	TotalFat      float64 `json:"total_fat"`
	MealCount     int     `json:"meal_count"`
	Meals         []Meal  `json:"meals"`
}
