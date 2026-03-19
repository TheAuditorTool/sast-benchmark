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
	ErrWorkoutNotFound  = errors.New("workout not found")
	ErrNotYourWorkout   = errors.New("workout does not belong to you")
	ErrExerciseNotFound = errors.New("exercise not found")
)

// WorkoutService handles workout business logic
type WorkoutService struct {
	repo *repository.WorkoutRepository
}

// NewWorkoutService creates a new workout service
func NewWorkoutService(repo *repository.WorkoutRepository) *WorkoutService {
	return &WorkoutService{repo: repo}
}

// Create creates a new workout
// TAINT FLOW: request -> service -> repository -> database
func (s *WorkoutService) Create(userID string, req models.CreateWorkoutRequest) (*models.Workout, error) {
	// Validate request
	if errs := validation.ValidateStruct(&req); errs != nil {
		return nil, &ValidationError{Errors: errs}
	}

	// Verify exercise exists
	exercise, err := s.repo.FindExerciseByID(req.ExerciseID)
	if err != nil {
		return nil, ErrExerciseNotFound
	}

	// Parse performed_at
	performedAt, err := time.Parse(time.RFC3339, req.PerformedAt)
	if err != nil {
		performedAt = time.Now()
	}

	workout := &models.Workout{
		UserID:          userID,
		ExerciseID:      req.ExerciseID,
		ExerciseName:    exercise.Name,
		DurationMinutes: req.DurationMinutes,
		Intensity:       models.WorkoutIntensity(req.Intensity),
		CaloriesBurned:  req.CaloriesBurned,
		Sets:            req.Sets,
		Reps:            req.Reps,
		WeightKg:        req.WeightKg,
		Notes:           req.Notes,
		PerformedAt:     performedAt,
	}

	if err := s.repo.Create(workout); err != nil {
		return nil, err
	}

	return workout, nil
}

// FindByID finds a workout by ID
func (s *WorkoutService) FindByID(id string) (*models.Workout, error) {
	workout, err := s.repo.FindByID(id)
	if err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, ErrWorkoutNotFound
		}
		return nil, err
	}
	return workout, nil
}

// Update updates a workout
func (s *WorkoutService) Update(workoutID, userID string, req models.UpdateWorkoutRequest) (*models.Workout, error) {
	workout, err := s.FindByID(workoutID)
	if err != nil {
		return nil, err
	}

	// Check ownership
	if workout.UserID != userID {
		return nil, ErrNotYourWorkout
	}

	// Update fields if provided
	if req.DurationMinutes != nil {
		workout.DurationMinutes = *req.DurationMinutes
	}
	if req.Intensity != nil {
		workout.Intensity = models.WorkoutIntensity(*req.Intensity)
	}
	if req.CaloriesBurned != nil {
		workout.CaloriesBurned = *req.CaloriesBurned
	}
	if req.Sets != nil {
		workout.Sets = req.Sets
	}
	if req.Reps != nil {
		workout.Reps = req.Reps
	}
	if req.WeightKg != nil {
		workout.WeightKg = req.WeightKg
	}
	if req.Notes != nil {
		workout.Notes = *req.Notes
	}

	if err := s.repo.Update(workout); err != nil {
		return nil, err
	}

	return workout, nil
}

// Delete deletes a workout
func (s *WorkoutService) Delete(workoutID, userID string) error {
	workout, err := s.FindByID(workoutID)
	if err != nil {
		return err
	}

	// Check ownership
	if workout.UserID != userID {
		return ErrNotYourWorkout
	}

	return s.repo.Delete(workoutID)
}

// ListForUser lists workouts for a user
// TAINT FLOW: pagination.BuildOrderBy() -> repository -> SQL ORDER BY
func (s *WorkoutService) ListForUser(userID string, pagination *models.PaginationParams) ([]models.Workout, error) {
	limit := pagination.GetLimit()
	offset := pagination.GetOffset()
	orderBy := pagination.BuildOrderBy("performed_at")

	return s.repo.ListForUser(userID, limit, offset, orderBy)
}

// GetDailySummary gets daily workout summary
func (s *WorkoutService) GetDailySummary(userID, date string) (*models.DailyWorkoutSummary, error) {
	return s.repo.GetDailySummary(userID, date)
}

// ListExercises lists all available exercises
func (s *WorkoutService) ListExercises() ([]models.Exercise, error) {
	return s.repo.ListExercises()
}

// EstimateCalories estimates calories burned for a workout
func (s *WorkoutService) EstimateCalories(exerciseID string, durationMinutes int, intensity string) (int, error) {
	exercise, err := s.repo.FindExerciseByID(exerciseID)
	if err != nil {
		return 0, ErrExerciseNotFound
	}

	baseCalories := float64(durationMinutes) * exercise.CaloriesPerMinute

	// Apply intensity multiplier
	multiplier := 1.0
	switch intensity {
	case "low":
		multiplier = 0.8
	case "moderate":
		multiplier = 1.0
	case "high":
		multiplier = 1.3
	case "very_high":
		multiplier = 1.5
	}

	return int(baseCalories * multiplier), nil
}
