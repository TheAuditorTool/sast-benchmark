package repository

import (
	"fmt"
	"time"

	"github.com/example/calorie-tracker/internal/models"
	"gorm.io/gorm"
)

// WorkoutRepository handles workout database operations
type WorkoutRepository struct {
	db *gorm.DB
}

// NewWorkoutRepository creates a new workout repository
func NewWorkoutRepository(db *gorm.DB) *WorkoutRepository {
	return &WorkoutRepository{db: db}
}

// Create creates a new workout
func (r *WorkoutRepository) Create(workout *models.Workout) error {
	return r.db.Create(workout).Error
}

// FindByID finds a workout by ID
func (r *WorkoutRepository) FindByID(id string) (*models.Workout, error) {
	var workout models.Workout
	err := r.db.Preload("Exercise").First(&workout, "id = ?", id).Error
	if err != nil {
		return nil, err
	}
	return &workout, nil
}

// Update updates a workout
func (r *WorkoutRepository) Update(workout *models.Workout) error {
	return r.db.Save(workout).Error
}

// Delete deletes a workout
func (r *WorkoutRepository) Delete(id string) error {
	return r.db.Delete(&models.Workout{}, "id = ?", id).Error
}

// ListForUser lists workouts for a user with pagination
func (r *WorkoutRepository) ListForUser(userID string, limit, offset int, orderBy string) ([]models.Workout, error) {
	var workouts []models.Workout
	query := r.db.Preload("Exercise").Where("user_id = ?", userID).Limit(limit).Offset(offset)

	if orderBy != "" {
		query = query.Order(orderBy)
	} else {
		query = query.Order("performed_at DESC")
	}

	err := query.Find(&workouts).Error

	// Populate exercise name
	for i := range workouts {
		if workouts[i].Exercise != nil {
			workouts[i].ExerciseName = workouts[i].Exercise.Name
		}
	}

	return workouts, err
}

// GetDailyWorkouts gets workouts for a specific date
func (r *WorkoutRepository) GetDailyWorkouts(userID, date string) ([]models.Workout, error) {
	var workouts []models.Workout
	err := r.db.Preload("Exercise").
		Where("user_id = ? AND DATE(performed_at) = ?", userID, date).
		Order("performed_at ASC").
		Find(&workouts).Error

	// Populate exercise name
	for i := range workouts {
		if workouts[i].Exercise != nil {
			workouts[i].ExerciseName = workouts[i].Exercise.Name
		}
	}

	return workouts, err
}

// GetDailySummary calculates daily workout totals
func (r *WorkoutRepository) GetDailySummary(userID, date string) (*models.DailyWorkoutSummary, error) {
	workouts, err := r.GetDailyWorkouts(userID, date)
	if err != nil {
		return nil, err
	}

	summary := &models.DailyWorkoutSummary{
		Date:          date,
		Workouts:      workouts,
		TotalWorkouts: len(workouts),
	}

	for _, workout := range workouts {
		summary.TotalDuration += workout.DurationMinutes
		summary.TotalCaloriesBurned += workout.CaloriesBurned
	}

	return summary, nil
}

// GetCaloriesBurnedForDateRange gets calories burned for a date range
func (r *WorkoutRepository) GetCaloriesBurnedForDateRange(userID string, startDate, endDate time.Time) (map[string]int, error) {
	type Result struct {
		Date           string
		CaloriesBurned int
	}
	var results []Result

	err := r.db.Model(&models.Workout{}).
		Select("DATE(performed_at) as date, SUM(calories_burned) as calories_burned").
		Where("user_id = ? AND performed_at BETWEEN ? AND ?", userID, startDate, endDate).
		Group("DATE(performed_at)").
		Scan(&results).Error

	if err != nil {
		return nil, err
	}

	burnedMap := make(map[string]int)
	for _, r := range results {
		burnedMap[r.Date] = r.CaloriesBurned
	}
	return burnedMap, nil
}

// ListExercises lists all exercises
func (r *WorkoutRepository) ListExercises() ([]models.Exercise, error) {
	var exercises []models.Exercise
	err := r.db.Order("category, name").Find(&exercises).Error
	return exercises, err
}

// FindExerciseByID finds an exercise by ID
func (r *WorkoutRepository) FindExerciseByID(id string) (*models.Exercise, error) {
	var exercise models.Exercise
	err := r.db.First(&exercise, "id = ?", id).Error
	if err != nil {
		return nil, err
	}
	return &exercise, nil
}

// SearchExercisesVulnerable searches exercises by term
func (r *WorkoutRepository) SearchExercisesVulnerable(searchTerm string) ([]models.Exercise, error) {
	var exercises []models.Exercise
	query := fmt.Sprintf(
		"SELECT * FROM exercises WHERE name LIKE '%%%s%%' OR category LIKE '%%%s%%'",
		searchTerm, searchTerm,
	)
	err := r.db.Raw(query).Scan(&exercises).Error
	return exercises, err
}
