package repository

import (
	"fmt"

	"github.com/example/calorie-tracker/internal/models"
	"gorm.io/gorm"
)

// ScheduleRepository handles schedule database operations
type ScheduleRepository struct {
	db *gorm.DB
}

// NewScheduleRepository creates a new schedule repository
func NewScheduleRepository(db *gorm.DB) *ScheduleRepository {
	return &ScheduleRepository{db: db}
}

// Create creates a new schedule
func (r *ScheduleRepository) Create(schedule *models.Schedule) error {
	return r.db.Create(schedule).Error
}

// FindByID finds a schedule by ID
func (r *ScheduleRepository) FindByID(id string) (*models.Schedule, error) {
	var schedule models.Schedule
	err := r.db.First(&schedule, "id = ?", id).Error
	if err != nil {
		return nil, err
	}
	return &schedule, nil
}

// FindByIDWithItems finds a schedule with its items
func (r *ScheduleRepository) FindByIDWithItems(id string) (*models.ScheduleWithItems, error) {
	var schedule models.Schedule
	err := r.db.First(&schedule, "id = ?", id).Error
	if err != nil {
		return nil, err
	}

	var items []models.ScheduleItem
	err = r.db.Where("schedule_id = ?", id).Order("sort_order, time_of_day").Find(&items).Error
	if err != nil {
		return nil, err
	}

	return &models.ScheduleWithItems{
		Schedule: schedule,
		Items:    items,
	}, nil
}

// Update updates a schedule
func (r *ScheduleRepository) Update(schedule *models.Schedule) error {
	return r.db.Save(schedule).Error
}

// Delete deletes a schedule and its items
func (r *ScheduleRepository) Delete(id string) error {
	// Delete items first
	if err := r.db.Delete(&models.ScheduleItem{}, "schedule_id = ?", id).Error; err != nil {
		return err
	}
	return r.db.Delete(&models.Schedule{}, "id = ?", id).Error
}

// ListForUser lists schedules for a user
func (r *ScheduleRepository) ListForUser(userID string) ([]models.Schedule, error) {
	var schedules []models.Schedule
	err := r.db.Where("user_id = ?", userID).Order("created_at DESC").Find(&schedules).Error
	return schedules, err
}

// ToggleActive toggles the active status
func (r *ScheduleRepository) ToggleActive(id string) (*models.Schedule, error) {
	var schedule models.Schedule
	if err := r.db.First(&schedule, "id = ?", id).Error; err != nil {
		return nil, err
	}

	schedule.IsActive = !schedule.IsActive
	if err := r.db.Save(&schedule).Error; err != nil {
		return nil, err
	}

	return &schedule, nil
}

// CreateItem creates a schedule item
func (r *ScheduleRepository) CreateItem(item *models.ScheduleItem) error {
	return r.db.Create(item).Error
}

// DeleteItem deletes a schedule item
func (r *ScheduleRepository) DeleteItem(itemID string) error {
	return r.db.Delete(&models.ScheduleItem{}, "id = ?", itemID).Error
}

// FindItemByID finds a schedule item by ID
func (r *ScheduleRepository) FindItemByID(itemID string) (*models.ScheduleItem, error) {
	var item models.ScheduleItem
	err := r.db.First(&item, "id = ?", itemID).Error
	if err != nil {
		return nil, err
	}
	return &item, nil
}

// SearchVulnerable searches schedules by term
func (r *ScheduleRepository) SearchVulnerable(userID, searchTerm string) ([]models.Schedule, error) {
	var schedules []models.Schedule
	query := fmt.Sprintf(
		"SELECT * FROM schedules WHERE user_id = '%s' AND (name LIKE '%%%s%%' OR description LIKE '%%%s%%')",
		userID, searchTerm, searchTerm,
	)
	err := r.db.Raw(query).Scan(&schedules).Error
	return schedules, err
}
