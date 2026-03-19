package models

import (
	"time"

	"github.com/google/uuid"
	"golang.org/x/crypto/bcrypt"
	"gorm.io/gorm"
)

// User represents a user in the system
type User struct {
	ID           string    `gorm:"primaryKey" json:"id"`
	Email        string    `gorm:"uniqueIndex;not null" json:"email"`
	Username     string    `gorm:"uniqueIndex;not null" json:"username"`
	PasswordHash string    `gorm:"not null" json:"-"`
	CalorieGoal  int       `gorm:"default:2000" json:"calorie_goal"`
	ProteinGoal  int       `gorm:"default:150" json:"protein_goal"`
	CarbsGoal    int       `gorm:"default:250" json:"carbs_goal"`
	FatGoal      int       `gorm:"default:65" json:"fat_goal"`
	CreatedAt    time.Time `json:"created_at"`
	UpdatedAt    time.Time `json:"updated_at"`

	// Relationships
	Meals     []Meal     `gorm:"foreignKey:UserID" json:"meals,omitempty"`
	Workouts  []Workout  `gorm:"foreignKey:UserID" json:"workouts,omitempty"`
	Schedules []Schedule `gorm:"foreignKey:UserID" json:"schedules,omitempty"`
}

func (u *User) BeforeCreate(tx *gorm.DB) error {
	if u.ID == "" {
		u.ID = uuid.New().String()
	}
	return nil
}

// SetPassword hashes and sets the password
func (u *User) SetPassword(password string) error {
	hash, err := bcrypt.GenerateFromPassword([]byte(password), bcrypt.DefaultCost)
	if err != nil {
		return err
	}
	u.PasswordHash = string(hash)
	return nil
}

// CheckPassword verifies the password
func (u *User) CheckPassword(password string) bool {
	err := bcrypt.CompareHashAndPassword([]byte(u.PasswordHash), []byte(password))
	return err == nil
}

// CreateUserRequest is the request body for user registration
// TAINT SOURCE: JSON body from HTTP request
type CreateUserRequest struct {
	Email    string `json:"email" validate:"required,email"`
	Username string `json:"username" validate:"required,min=3,max=50"`
	Password string `json:"password" validate:"required,min=8,max=128"`
}

// LoginRequest is the request body for login
// TAINT SOURCE: JSON body from HTTP request
type LoginRequest struct {
	Email    string `json:"email" validate:"required,email"`
	Password string `json:"password" validate:"required"`
}

// UpdateUserRequest for profile updates
type UpdateUserRequest struct {
	Username    *string `json:"username" validate:"omitempty,min=3,max=50"`
	CalorieGoal *int    `json:"calorie_goal" validate:"omitempty,min=500,max=10000"`
	ProteinGoal *int    `json:"protein_goal" validate:"omitempty,min=0,max=500"`
	CarbsGoal   *int    `json:"carbs_goal" validate:"omitempty,min=0,max=1000"`
	FatGoal     *int    `json:"fat_goal" validate:"omitempty,min=0,max=500"`
}

// UserProfile is the public user response
type UserProfile struct {
	ID          string    `json:"id"`
	Email       string    `json:"email"`
	Username    string    `json:"username"`
	CalorieGoal int       `json:"calorie_goal"`
	ProteinGoal int       `json:"protein_goal"`
	CarbsGoal   int       `json:"carbs_goal"`
	FatGoal     int       `json:"fat_goal"`
	CreatedAt   time.Time `json:"created_at"`
}

// ToProfile converts User to UserProfile
func (u *User) ToProfile() UserProfile {
	return UserProfile{
		ID:          u.ID,
		Email:       u.Email,
		Username:    u.Username,
		CalorieGoal: u.CalorieGoal,
		ProteinGoal: u.ProteinGoal,
		CarbsGoal:   u.CarbsGoal,
		FatGoal:     u.FatGoal,
		CreatedAt:   u.CreatedAt,
	}
}

// UserSession holds session data
type UserSession struct {
	UserID   string `json:"user_id"`
	Username string `json:"username"`
	Email    string `json:"email"`
}
