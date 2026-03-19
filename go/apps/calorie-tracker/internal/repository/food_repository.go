package repository

import (
	"fmt"

	"github.com/example/calorie-tracker/internal/models"
	"gorm.io/gorm"
)

// FoodRepository handles food database operations
type FoodRepository struct {
	db *gorm.DB
}

// NewFoodRepository creates a new food repository
func NewFoodRepository(db *gorm.DB) *FoodRepository {
	return &FoodRepository{db: db}
}

// Create creates a new food
func (r *FoodRepository) Create(food *models.Food) error {
	food.IsCustom = true
	return r.db.Create(food).Error
}

// FindByID finds a food by ID
func (r *FoodRepository) FindByID(id string) (*models.Food, error) {
	var food models.Food
	err := r.db.First(&food, "id = ?", id).Error
	if err != nil {
		return nil, err
	}
	return &food, nil
}

// ListAll lists all foods
func (r *FoodRepository) ListAll() ([]models.Food, error) {
	var foods []models.Food
	err := r.db.Order("name").Find(&foods).Error
	return foods, err
}

// ListByCategory lists foods by category
func (r *FoodRepository) ListByCategory(category string) ([]models.Food, error) {
	var foods []models.Food
	err := r.db.Where("category = ?", category).Order("name").Find(&foods).Error
	return foods, err
}

// SearchVulnerable searches foods with SQL injection vulnerability
// TAINT SINK: User input directly in SQL query
func (r *FoodRepository) SearchVulnerable(searchTerm string) ([]models.Food, error) {
	var foods []models.Food
	// VULNERABLE: Direct string interpolation in SQL
	query := fmt.Sprintf(
		"SELECT * FROM foods WHERE name LIKE '%%%s%%' OR brand LIKE '%%%s%%' OR category LIKE '%%%s%%'",
		searchTerm, searchTerm, searchTerm,
	)
	err := r.db.Raw(query).Scan(&foods).Error
	return foods, err
}

// Delete deletes a food
func (r *FoodRepository) Delete(id string) error {
	return r.db.Delete(&models.Food{}, "id = ?", id).Error
}
