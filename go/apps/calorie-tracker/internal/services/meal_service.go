package services

import (
	"errors"
	"time"

	"github.com/example/calorie-tracker/internal/models"
	"github.com/example/calorie-tracker/internal/repository"
	"github.com/example/calorie-tracker/internal/validation"
	"gorm.io/gorm"
)

var (
	ErrMealNotFound = errors.New("meal not found")
	ErrNotYourMeal  = errors.New("meal does not belong to you")
)

// MealService handles meal business logic
type MealService struct {
	repo *repository.MealRepository
}

// NewMealService creates a new meal service
func NewMealService(repo *repository.MealRepository) *MealService {
	return &MealService{repo: repo}
}

// Create creates a new meal
func (s *MealService) Create(userID string, req models.CreateMealRequest) (*models.Meal, error) {
	// Validate request
	if errs := validation.ValidateStruct(&req); errs != nil {
		return nil, &ValidationError{Errors: errs}
	}

	// Parse consumed_at
	consumedAt, err := time.Parse(time.RFC3339, req.ConsumedAt)
	if err != nil {
		consumedAt = time.Now()
	}

	meal := &models.Meal{
		UserID:       userID,
		Name:         req.Name,
		Description:  req.Description,
		MealType:     models.MealType(req.MealType),
		Calories:     req.Calories,
		ProteinGrams: req.ProteinGrams,
		CarbsGrams:   req.CarbsGrams,
		FatGrams:     req.FatGrams,
		ConsumedAt:   consumedAt,
	}

	if err := s.repo.Create(meal); err != nil {
		return nil, err
	}

	return meal, nil
}

// FindByID finds a meal by ID
func (s *MealService) FindByID(id string) (*models.Meal, error) {
	meal, err := s.repo.FindByID(id)
	if err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, ErrMealNotFound
		}
		return nil, err
	}
	return meal, nil
}

// Update updates a meal
func (s *MealService) Update(mealID, userID string, req models.UpdateMealRequest) (*models.Meal, error) {
	meal, err := s.FindByID(mealID)
	if err != nil {
		return nil, err
	}

	// Check ownership
	if meal.UserID != userID {
		return nil, ErrNotYourMeal
	}

	// Update fields if provided
	if req.Name != nil {
		meal.Name = *req.Name
	}
	if req.Description != nil {
		meal.Description = *req.Description
	}
	if req.MealType != nil {
		meal.MealType = models.MealType(*req.MealType)
	}
	if req.Calories != nil {
		meal.Calories = *req.Calories
	}
	if req.ProteinGrams != nil {
		meal.ProteinGrams = req.ProteinGrams
	}
	if req.CarbsGrams != nil {
		meal.CarbsGrams = req.CarbsGrams
	}
	if req.FatGrams != nil {
		meal.FatGrams = req.FatGrams
	}

	if err := s.repo.Update(meal); err != nil {
		return nil, err
	}

	return meal, nil
}

// Delete deletes a meal
func (s *MealService) Delete(mealID, userID string) error {
	meal, err := s.FindByID(mealID)
	if err != nil {
		return err
	}

	// Check ownership
	if meal.UserID != userID {
		return ErrNotYourMeal
	}

	return s.repo.Delete(mealID)
}

// ListForUser lists meals for a user
func (s *MealService) ListForUser(userID string, pagination *models.PaginationParams, dateFilter *models.DateRangeFilter) ([]models.Meal, error) {
	limit := pagination.GetLimit()
	offset := pagination.GetOffset()
	orderBy := pagination.BuildOrderBy("consumed_at")

	if dateFilter != nil && (dateFilter.StartDate != "" || dateFilter.EndDate != "") {
		return s.repo.ListForUserWithDateRange(userID, dateFilter.StartDate, dateFilter.EndDate, limit, offset, orderBy)
	}

	return s.repo.ListForUser(userID, limit, offset, orderBy)
}

// GetDailySummary gets daily meal summary
func (s *MealService) GetDailySummary(userID, date string) (*models.DailyMealSummary, error) {
	return s.repo.GetDailySummary(userID, date)
}

// SearchMeals searches meals by term
func (s *MealService) SearchMeals(userID, searchTerm string) ([]models.Meal, error) {
	return s.repo.Search(userID, searchTerm)
}

// QuickLog creates a quick meal entry
func (s *MealService) QuickLog(userID string, entry models.QuickMealEntry) (*models.Meal, error) {
	mealType := entry.MealType
	if mealType == "" {
		mealType = "snack"
	}

	req := models.CreateMealRequest{
		Name:       entry.Name,
		MealType:   mealType,
		Calories:   entry.Calories,
		ConsumedAt: time.Now().Format(time.RFC3339),
	}

	return s.Create(userID, req)
}
