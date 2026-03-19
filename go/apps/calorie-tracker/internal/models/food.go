package models

import (
	"time"

	"github.com/google/uuid"
	"gorm.io/gorm"
)

// Food represents a food item in the database
type Food struct {
	ID                 string    `gorm:"primaryKey" json:"id"`
	Name               string    `gorm:"not null;index" json:"name"`
	Brand              string    `json:"brand,omitempty"`
	Category           string    `gorm:"index" json:"category"`
	ServingSize        float64   `gorm:"not null" json:"serving_size"`
	ServingUnit        string    `gorm:"not null" json:"serving_unit"`
	CaloriesPerServing int       `gorm:"not null" json:"calories_per_serving"`
	ProteinGrams       float64   `json:"protein_grams"`
	CarbsGrams         float64   `json:"carbs_grams"`
	FatGrams           float64   `json:"fat_grams"`
	FiberGrams         float64   `json:"fiber_grams"`
	SugarGrams         float64   `json:"sugar_grams"`
	SodiumMg           float64   `json:"sodium_mg"`
	IsCustom           bool      `gorm:"default:false" json:"is_custom"`
	CreatedAt          time.Time `json:"created_at"`
}

func (f *Food) BeforeCreate(tx *gorm.DB) error {
	if f.ID == "" {
		f.ID = uuid.New().String()
	}
	return nil
}

// CreateFoodRequest for creating custom foods
// TAINT SOURCE: JSON body
type CreateFoodRequest struct {
	Name               string  `json:"name" validate:"required,min=1,max=200"`
	Brand              string  `json:"brand" validate:"max=100"`
	Category           string  `json:"category" validate:"max=50"`
	ServingSize        float64 `json:"serving_size" validate:"required,min=0.1"`
	ServingUnit        string  `json:"serving_unit" validate:"required,max=20"`
	CaloriesPerServing int     `json:"calories_per_serving" validate:"required,min=0,max=10000"`
	ProteinGrams       float64 `json:"protein_grams" validate:"min=0,max=500"`
	CarbsGrams         float64 `json:"carbs_grams" validate:"min=0,max=1000"`
	FatGrams           float64 `json:"fat_grams" validate:"min=0,max=500"`
	FiberGrams         float64 `json:"fiber_grams" validate:"min=0,max=100"`
	SugarGrams         float64 `json:"sugar_grams" validate:"min=0,max=500"`
	SodiumMg           float64 `json:"sodium_mg" validate:"min=0,max=50000"`
}
