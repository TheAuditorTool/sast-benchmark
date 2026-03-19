package services

import (
	"errors"

	"github.com/example/calorie-tracker/internal/models"
	"github.com/example/calorie-tracker/internal/repository"
	"github.com/example/calorie-tracker/internal/validation"
	"gorm.io/gorm"
)

var (
	ErrScheduleNotFound = errors.New("schedule not found")
	ErrNotYourSchedule  = errors.New("schedule does not belong to you")
	ErrItemNotFound     = errors.New("schedule item not found")
)

// ScheduleService handles schedule business logic
type ScheduleService struct {
	repo *repository.ScheduleRepository
}

// NewScheduleService creates a new schedule service
func NewScheduleService(repo *repository.ScheduleRepository) *ScheduleService {
	return &ScheduleService{repo: repo}
}

// Create creates a new schedule
// TAINT FLOW: request -> service -> repository -> database
func (s *ScheduleService) Create(userID string, req models.CreateScheduleRequest) (*models.Schedule, error) {
	// Validate request
	if errs := validation.ValidateStruct(&req); errs != nil {
		return nil, &ValidationError{Errors: errs}
	}

	schedule := &models.Schedule{
		UserID:       userID,
		Name:         req.Name,
		Description:  req.Description,
		ScheduleType: models.ScheduleType(req.ScheduleType),
		DaysOfWeek:   req.DaysOfWeek,
		TimeOfDay:    req.TimeOfDay,
		IsActive:     true,
	}

	if err := s.repo.Create(schedule); err != nil {
		return nil, err
	}

	return schedule, nil
}

// FindByID finds a schedule by ID
func (s *ScheduleService) FindByID(id string) (*models.Schedule, error) {
	schedule, err := s.repo.FindByID(id)
	if err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, ErrScheduleNotFound
		}
		return nil, err
	}
	return schedule, nil
}

// GetWithItems gets a schedule with its items
func (s *ScheduleService) GetWithItems(scheduleID string) (*models.ScheduleWithItems, error) {
	return s.repo.FindByIDWithItems(scheduleID)
}

// Update updates a schedule
func (s *ScheduleService) Update(scheduleID, userID string, req models.UpdateScheduleRequest) (*models.Schedule, error) {
	schedule, err := s.FindByID(scheduleID)
	if err != nil {
		return nil, err
	}

	// Check ownership
	if schedule.UserID != userID {
		return nil, ErrNotYourSchedule
	}

	// Update fields if provided
	if req.Name != nil {
		schedule.Name = *req.Name
	}
	if req.Description != nil {
		schedule.Description = *req.Description
	}
	if req.DaysOfWeek != nil {
		schedule.DaysOfWeek = *req.DaysOfWeek
	}
	if req.TimeOfDay != nil {
		schedule.TimeOfDay = *req.TimeOfDay
	}
	if req.IsActive != nil {
		schedule.IsActive = *req.IsActive
	}

	if err := s.repo.Update(schedule); err != nil {
		return nil, err
	}

	return schedule, nil
}

// Delete deletes a schedule
func (s *ScheduleService) Delete(scheduleID, userID string) error {
	schedule, err := s.FindByID(scheduleID)
	if err != nil {
		return err
	}

	// Check ownership
	if schedule.UserID != userID {
		return ErrNotYourSchedule
	}

	return s.repo.Delete(scheduleID)
}

// ListForUser lists schedules for a user
func (s *ScheduleService) ListForUser(userID string) ([]models.Schedule, error) {
	return s.repo.ListForUser(userID)
}

// ToggleActive toggles the active status of a schedule
func (s *ScheduleService) ToggleActive(scheduleID, userID string) (*models.Schedule, error) {
	schedule, err := s.FindByID(scheduleID)
	if err != nil {
		return nil, err
	}

	// Check ownership
	if schedule.UserID != userID {
		return nil, ErrNotYourSchedule
	}

	return s.repo.ToggleActive(scheduleID)
}

// AddItem adds an item to a schedule
func (s *ScheduleService) AddItem(scheduleID string, req models.ScheduleItemRequest) (*models.ScheduleItem, error) {
	// Validate request
	if errs := validation.ValidateStruct(&req); errs != nil {
		return nil, &ValidationError{Errors: errs}
	}

	item := &models.ScheduleItem{
		ScheduleID:     scheduleID,
		ItemType:       req.ItemType,
		Name:           req.Name,
		TimeOfDay:      req.TimeOfDay,
		TargetCalories: req.TargetCalories,
		Notes:          req.Notes,
	}

	if err := s.repo.CreateItem(item); err != nil {
		return nil, err
	}

	return item, nil
}

// DeleteItem deletes a schedule item
func (s *ScheduleService) DeleteItem(itemID, userID string) error {
	// Find item to get schedule ID
	item, err := s.repo.FindItemByID(itemID)
	if err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return ErrItemNotFound
		}
		return err
	}

	// Verify ownership through schedule
	schedule, err := s.FindByID(item.ScheduleID)
	if err != nil {
		return err
	}

	if schedule.UserID != userID {
		return ErrNotYourSchedule
	}

	return s.repo.DeleteItem(itemID)
}
