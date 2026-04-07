package services

import (
	"errors"

	"github.com/example/calorie-tracker/internal/models"
	"github.com/example/calorie-tracker/internal/repository"
	"github.com/example/calorie-tracker/internal/validation"
	"gorm.io/gorm"
)

var (
	ErrFoodNotFound = errors.New("food not found")
)

// FoodService handles food business logic
type FoodService struct {
	repo *repository.FoodRepository
}

// NewFoodService creates a new food service
func NewFoodService(repo *repository.FoodRepository) *FoodService {
	return &FoodService{repo: repo}
}

// Create creates a new food
func (s *FoodService) Create(req models.CreateFoodRequest) (*models.Food, error) {
	// Validate request
	if errs := validation.ValidateStruct(&req); errs != nil {
		return nil, &ValidationError{Errors: errs}
	}

	food := &models.Food{
		Name:               req.Name,
		Brand:              req.Brand,
		Category:           req.Category,
		ServingSize:        req.ServingSize,
		ServingUnit:        req.ServingUnit,
		CaloriesPerServing: req.CaloriesPerServing,
		ProteinGrams:       req.ProteinGrams,
		CarbsGrams:         req.CarbsGrams,
		FatGrams:           req.FatGrams,
		FiberGrams:         req.FiberGrams,
		SugarGrams:         req.SugarGrams,
		SodiumMg:           req.SodiumMg,
		IsCustom:           true,
	}

	if err := s.repo.Create(food); err != nil {
		return nil, err
	}

	return food, nil
}

// FindByID finds a food by ID
func (s *FoodService) FindByID(id string) (*models.Food, error) {
	food, err := s.repo.FindByID(id)
	if err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, ErrFoodNotFound
		}
		return nil, err
	}
	return food, nil
}

// ListAll lists all foods
func (s *FoodService) ListAll() ([]models.Food, error) {
	return s.repo.ListAll()
}

// ListByCategory lists foods by category
func (s *FoodService) ListByCategory(category string) ([]models.Food, error) {
	return s.repo.ListByCategory(category)
}

// Search searches foods by term
func (s *FoodService) Search(searchTerm string) ([]models.Food, error) {
	return s.repo.Search(searchTerm)
}
