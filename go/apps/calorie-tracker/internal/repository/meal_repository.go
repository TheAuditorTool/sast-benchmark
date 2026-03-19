package repository

import (
	"fmt"
	"time"

	"github.com/example/calorie-tracker/internal/models"
	"gorm.io/gorm"
)

// MealRepository handles meal database operations
type MealRepository struct {
	db *gorm.DB
}

// NewMealRepository creates a new meal repository
func NewMealRepository(db *gorm.DB) *MealRepository {
	return &MealRepository{db: db}
}

// Create creates a new meal
func (r *MealRepository) Create(meal *models.Meal) error {
	return r.db.Create(meal).Error
}

// FindByID finds a meal by ID
func (r *MealRepository) FindByID(id string) (*models.Meal, error) {
	var meal models.Meal
	err := r.db.First(&meal, "id = ?", id).Error
	if err != nil {
		return nil, err
	}
	return &meal, nil
}

// Update updates a meal
func (r *MealRepository) Update(meal *models.Meal) error {
	return r.db.Save(meal).Error
}

// Delete deletes a meal
func (r *MealRepository) Delete(id string) error {
	return r.db.Delete(&models.Meal{}, "id = ?", id).Error
}

// ListForUser lists meals for a user with pagination
// TAINT PROPAGATION: orderBy from user input flows to SQL
func (r *MealRepository) ListForUser(userID string, limit, offset int, orderBy string) ([]models.Meal, error) {
	var meals []models.Meal
	query := r.db.Where("user_id = ?", userID).Limit(limit).Offset(offset)

	if orderBy != "" {
		// VULNERABLE: Order by clause from user input
		query = query.Order(orderBy)
	} else {
		query = query.Order("consumed_at DESC")
	}

	err := query.Find(&meals).Error
	return meals, err
}

// ListForUserWithDateRange lists meals for a user within a date range
func (r *MealRepository) ListForUserWithDateRange(userID, startDate, endDate string, limit, offset int, orderBy string) ([]models.Meal, error) {
	var meals []models.Meal
	query := r.db.Where("user_id = ?", userID)

	if startDate != "" {
		query = query.Where("DATE(consumed_at) >= ?", startDate)
	}
	if endDate != "" {
		query = query.Where("DATE(consumed_at) <= ?", endDate)
	}

	query = query.Limit(limit).Offset(offset)

	if orderBy != "" {
		query = query.Order(orderBy)
	} else {
		query = query.Order("consumed_at DESC")
	}

	err := query.Find(&meals).Error
	return meals, err
}

// GetDailyMeals gets meals for a specific date
func (r *MealRepository) GetDailyMeals(userID, date string) ([]models.Meal, error) {
	var meals []models.Meal
	err := r.db.Where("user_id = ? AND DATE(consumed_at) = ?", userID, date).
		Order("consumed_at ASC").
		Find(&meals).Error
	return meals, err
}

// GetDailySummary calculates daily meal totals
func (r *MealRepository) GetDailySummary(userID, date string) (*models.DailyMealSummary, error) {
	meals, err := r.GetDailyMeals(userID, date)
	if err != nil {
		return nil, err
	}

	summary := &models.DailyMealSummary{
		Date:   date,
		Meals:  meals,
		MealCount: len(meals),
	}

	for _, meal := range meals {
		summary.TotalCalories += meal.Calories
		if meal.ProteinGrams != nil {
			summary.TotalProtein += *meal.ProteinGrams
		}
		if meal.CarbsGrams != nil {
			summary.TotalCarbs += *meal.CarbsGrams
		}
		if meal.FatGrams != nil {
			summary.TotalFat += *meal.FatGrams
		}
	}

	return summary, nil
}

// SearchVulnerable searches meals with SQL injection vulnerability
// TAINT SINK: User input directly in SQL query
func (r *MealRepository) SearchVulnerable(userID, searchTerm string) ([]models.Meal, error) {
	var meals []models.Meal
	// VULNERABLE: Direct string interpolation in SQL
	query := fmt.Sprintf(
		"SELECT * FROM meals WHERE user_id = '%s' AND (name LIKE '%%%s%%' OR description LIKE '%%%s%%')",
		userID, searchTerm, searchTerm,
	)
	err := r.db.Raw(query).Scan(&meals).Error
	return meals, err
}

// GetCaloriesForDateRange gets calorie totals for a date range
func (r *MealRepository) GetCaloriesForDateRange(userID string, startDate, endDate time.Time) (map[string]int, error) {
	type Result struct {
		Date     string
		Calories int
	}
	var results []Result

	err := r.db.Model(&models.Meal{}).
		Select("DATE(consumed_at) as date, SUM(calories) as calories").
		Where("user_id = ? AND consumed_at BETWEEN ? AND ?", userID, startDate, endDate).
		Group("DATE(consumed_at)").
		Scan(&results).Error

	if err != nil {
		return nil, err
	}

	calorieMap := make(map[string]int)
	for _, r := range results {
		calorieMap[r.Date] = r.Calories
	}
	return calorieMap, nil
}
