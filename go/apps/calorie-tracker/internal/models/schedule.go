package models

import (
	"time"

	"github.com/google/uuid"
	"gorm.io/gorm"
)

// ScheduleType represents the type of schedule
type ScheduleType string

const (
	ScheduleTypeMealPlan    ScheduleType = "meal_plan"
	ScheduleTypeWorkoutPlan ScheduleType = "workout_plan"
	ScheduleTypeCombined    ScheduleType = "combined"
)

// Schedule represents a meal or workout schedule
type Schedule struct {
	ID           string       `gorm:"primaryKey" json:"id"`
	UserID       string       `gorm:"not null;index" json:"user_id"`
	Name         string       `gorm:"not null" json:"name"`
	Description  string       `json:"description,omitempty"`
	ScheduleType ScheduleType `gorm:"not null" json:"schedule_type"`
	DaysOfWeek   string       `json:"days_of_week,omitempty"` // comma-separated: monday,tuesday,etc
	TimeOfDay    string       `json:"time_of_day,omitempty"`  // HH:MM format
	IsActive     bool         `gorm:"default:true" json:"is_active"`
	CreatedAt    time.Time    `json:"created_at"`
	UpdatedAt    time.Time    `json:"updated_at"`

	// Relationships
	User  *User          `gorm:"foreignKey:UserID" json:"user,omitempty"`
	Items []ScheduleItem `gorm:"foreignKey:ScheduleID" json:"items,omitempty"`
}

func (s *Schedule) BeforeCreate(tx *gorm.DB) error {
	if s.ID == "" {
		s.ID = uuid.New().String()
	}
	return nil
}

// ScheduleItem represents an item in a schedule
type ScheduleItem struct {
	ID             string    `gorm:"primaryKey" json:"id"`
	ScheduleID     string    `gorm:"not null;index" json:"schedule_id"`
	ItemType       string    `gorm:"not null" json:"item_type"` // meal or workout
	Name           string    `gorm:"not null" json:"name"`
	TimeOfDay      string    `gorm:"not null" json:"time_of_day"` // HH:MM
	TargetCalories *int      `json:"target_calories,omitempty"`
	Notes          string    `json:"notes,omitempty"`
	SortOrder      int       `gorm:"default:0" json:"sort_order"`
	CreatedAt      time.Time `json:"created_at"`

	// Relationships
	Schedule *Schedule `gorm:"foreignKey:ScheduleID" json:"schedule,omitempty"`
}

func (si *ScheduleItem) BeforeCreate(tx *gorm.DB) error {
	if si.ID == "" {
		si.ID = uuid.New().String()
	}
	return nil
}

// CreateScheduleRequest for creating a schedule
// TAINT SOURCE: JSON body
type CreateScheduleRequest struct {
	Name         string `json:"name" validate:"required,min=1,max=200"`
	Description  string `json:"description" validate:"max=1000"`
	ScheduleType string `json:"schedule_type" validate:"required,oneof=meal_plan workout_plan combined"`
	DaysOfWeek   string `json:"days_of_week" validate:"max=100"`
	TimeOfDay    string `json:"time_of_day" validate:"max=10"`
}

// UpdateScheduleRequest for updating a schedule
type UpdateScheduleRequest struct {
	Name        *string `json:"name" validate:"omitempty,min=1,max=200"`
	Description *string `json:"description" validate:"omitempty,max=1000"`
	DaysOfWeek  *string `json:"days_of_week" validate:"omitempty,max=100"`
	TimeOfDay   *string `json:"time_of_day" validate:"omitempty,max=10"`
	IsActive    *bool   `json:"is_active"`
}

// ScheduleItemRequest for adding items to a schedule
type ScheduleItemRequest struct {
	ItemType       string `json:"item_type" validate:"required,oneof=meal workout"`
	Name           string `json:"name" validate:"required,min=1,max=200"`
	TimeOfDay      string `json:"time_of_day" validate:"required,max=10"`
	TargetCalories *int   `json:"target_calories" validate:"omitempty,min=1,max=10000"`
	Notes          string `json:"notes" validate:"max=500"`
}

// ScheduleWithItems combines schedule with its items
type ScheduleWithItems struct {
	Schedule Schedule       `json:"schedule"`
	Items    []ScheduleItem `json:"items"`
}
