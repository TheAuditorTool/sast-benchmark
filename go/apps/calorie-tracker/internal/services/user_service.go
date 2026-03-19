package services

import (
	"errors"

	"github.com/example/calorie-tracker/internal/models"
	"github.com/example/calorie-tracker/internal/repository"
	"github.com/example/calorie-tracker/internal/validation"
	"gorm.io/gorm"
)

var (
	ErrUserNotFound       = errors.New("user not found")
	ErrEmailExists        = errors.New("email already exists")
	ErrUsernameExists     = errors.New("username already exists")
	ErrInvalidCredentials = errors.New("invalid credentials")
)

// UserService handles user business logic
type UserService struct {
	repo *repository.UserRepository
}

// NewUserService creates a new user service
func NewUserService(repo *repository.UserRepository) *UserService {
	return &UserService{repo: repo}
}

// Register registers a new user
// TAINT FLOW: request -> validation -> repository -> database
func (s *UserService) Register(req models.CreateUserRequest) (*models.User, error) {
	// Validate request
	if errs := validation.ValidateStruct(&req); errs != nil {
		return nil, &ValidationError{Errors: errs}
	}

	// Check if email exists
	exists, _ := s.repo.ExistsWithEmail(req.Email)
	if exists {
		return nil, ErrEmailExists
	}

	// Check if username exists
	exists, _ = s.repo.ExistsWithUsername(req.Username)
	if exists {
		return nil, ErrUsernameExists
	}

	// Create user
	user := &models.User{
		Email:    req.Email,
		Username: req.Username,
	}

	// Set password
	if err := user.SetPassword(req.Password); err != nil {
		return nil, err
	}

	// Save to database
	if err := s.repo.Create(user); err != nil {
		return nil, err
	}

	return user, nil
}

// Login authenticates a user
// TAINT FLOW: request -> service -> repository -> session
func (s *UserService) Login(req models.LoginRequest) (*models.UserSession, error) {
	// Validate request
	if errs := validation.ValidateStruct(&req); errs != nil {
		return nil, &ValidationError{Errors: errs}
	}

	// Find user by email
	user, err := s.repo.FindByEmail(req.Email)
	if err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, ErrInvalidCredentials
		}
		return nil, err
	}

	// Check password
	if !user.CheckPassword(req.Password) {
		return nil, ErrInvalidCredentials
	}

	// Create session
	session := &models.UserSession{
		UserID:   user.ID,
		Username: user.Username,
		Email:    user.Email,
	}

	return session, nil
}

// FindByID finds a user by ID
func (s *UserService) FindByID(id string) (*models.User, error) {
	user, err := s.repo.FindByID(id)
	if err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, ErrUserNotFound
		}
		return nil, err
	}
	return user, nil
}

// Update updates a user
func (s *UserService) Update(userID string, req models.UpdateUserRequest) (*models.User, error) {
	user, err := s.FindByID(userID)
	if err != nil {
		return nil, err
	}

	// Update fields if provided
	if req.Username != nil {
		// Check if username is taken by another user
		existing, _ := s.repo.FindByUsername(*req.Username)
		if existing != nil && existing.ID != userID {
			return nil, ErrUsernameExists
		}
		user.Username = *req.Username
	}

	if req.CalorieGoal != nil {
		user.CalorieGoal = *req.CalorieGoal
	}
	if req.ProteinGoal != nil {
		user.ProteinGoal = *req.ProteinGoal
	}
	if req.CarbsGoal != nil {
		user.CarbsGoal = *req.CarbsGoal
	}
	if req.FatGoal != nil {
		user.FatGoal = *req.FatGoal
	}

	if err := s.repo.Update(user); err != nil {
		return nil, err
	}

	return user, nil
}

// SearchUsersVulnerable searches users - VULNERABLE
// TAINT FLOW: searchTerm -> repository.SearchVulnerable -> SQL
func (s *UserService) SearchUsersVulnerable(searchTerm string) ([]models.User, error) {
	// VULNERABLE: Search term flows to SQL query
	return s.repo.SearchVulnerable(searchTerm)
}

// ValidationError represents validation errors
type ValidationError struct {
	Errors []string
}

func (e *ValidationError) Error() string {
	if len(e.Errors) > 0 {
		return e.Errors[0]
	}
	return "validation error"
}
