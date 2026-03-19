package handlers

import (
	"errors"
	"net/http"

	"github.com/gin-gonic/gin"

	"github.com/example/calorie-tracker/internal/middleware"
	"github.com/example/calorie-tracker/internal/models"
	"github.com/example/calorie-tracker/internal/services"
)

// WorkoutHandler handles workout endpoints
type WorkoutHandler struct {
	workoutService *services.WorkoutService
}

// NewWorkoutHandler creates a new workout handler
func NewWorkoutHandler(workoutService *services.WorkoutService) *WorkoutHandler {
	return &WorkoutHandler{workoutService: workoutService}
}

// ListWorkouts lists workouts for the current user
// GET /api/workouts
// TAINT SOURCE: Query params -> service -> repository -> SQL
func (h *WorkoutHandler) ListWorkouts(c *gin.Context) {
	userID := middleware.GetUserID(c)

	var pagination models.PaginationParams
	if err := c.ShouldBindQuery(&pagination); err != nil {
		pagination = models.PaginationParams{Page: 1, PerPage: 20}
	}

	workouts, err := h.workoutService.ListForUser(userID, &pagination)
	if err != nil {
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to list workouts"))
		return
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(workouts))
}

// CreateWorkout creates a new workout
// POST /api/workouts
// TAINT SOURCE: JSON body -> service -> repository -> database
func (h *WorkoutHandler) CreateWorkout(c *gin.Context) {
	userID := middleware.GetUserID(c)

	var req models.CreateWorkoutRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, models.NewErrorResponse("Invalid request body"))
		return
	}

	workout, err := h.workoutService.Create(userID, req)
	if err != nil {
		if errors.Is(err, services.ErrExerciseNotFound) {
			c.JSON(http.StatusBadRequest, models.NewErrorResponse("Exercise not found"))
			return
		}
		if validationErr, ok := err.(*services.ValidationError); ok {
			c.JSON(http.StatusBadRequest, models.NewValidationErrorResponse(validationErr.Errors))
			return
		}
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to create workout"))
		return
	}

	c.JSON(http.StatusCreated, models.NewSuccessResponse(workout))
}

// GetWorkout gets a specific workout
// GET /api/workouts/:id
func (h *WorkoutHandler) GetWorkout(c *gin.Context) {
	userID := middleware.GetUserID(c)
	workoutID := c.Param("id")

	workout, err := h.workoutService.FindByID(workoutID)
	if err != nil {
		if errors.Is(err, services.ErrWorkoutNotFound) {
			c.JSON(http.StatusNotFound, models.NewErrorResponse("Workout not found"))
			return
		}
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to get workout"))
		return
	}

	// Verify ownership
	if workout.UserID != userID {
		c.JSON(http.StatusForbidden, models.NewErrorResponse("Not your workout"))
		return
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(workout))
}

// UpdateWorkout updates a workout
// PUT /api/workouts/:id
func (h *WorkoutHandler) UpdateWorkout(c *gin.Context) {
	userID := middleware.GetUserID(c)
	workoutID := c.Param("id")

	var req models.UpdateWorkoutRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, models.NewErrorResponse("Invalid request body"))
		return
	}

	workout, err := h.workoutService.Update(workoutID, userID, req)
	if err != nil {
		if errors.Is(err, services.ErrWorkoutNotFound) {
			c.JSON(http.StatusNotFound, models.NewErrorResponse("Workout not found"))
			return
		}
		if errors.Is(err, services.ErrNotYourWorkout) {
			c.JSON(http.StatusForbidden, models.NewErrorResponse("Not your workout"))
			return
		}
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to update workout"))
		return
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(workout))
}

// DeleteWorkout deletes a workout
// DELETE /api/workouts/:id
func (h *WorkoutHandler) DeleteWorkout(c *gin.Context) {
	userID := middleware.GetUserID(c)
	workoutID := c.Param("id")

	err := h.workoutService.Delete(workoutID, userID)
	if err != nil {
		if errors.Is(err, services.ErrWorkoutNotFound) {
			c.JSON(http.StatusNotFound, models.NewErrorResponse("Workout not found"))
			return
		}
		if errors.Is(err, services.ErrNotYourWorkout) {
			c.JSON(http.StatusForbidden, models.NewErrorResponse("Not your workout"))
			return
		}
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to delete workout"))
		return
	}

	c.Status(http.StatusNoContent)
}

// GetDailyWorkouts gets workouts for a specific date
// GET /api/workouts/daily/:date
func (h *WorkoutHandler) GetDailyWorkouts(c *gin.Context) {
	userID := middleware.GetUserID(c)
	date := c.Param("date")

	summary, err := h.workoutService.GetDailySummary(userID, date)
	if err != nil {
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to get daily workouts"))
		return
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(summary))
}

// ListExercises lists all available exercises
// GET /api/exercises
func (h *WorkoutHandler) ListExercises(c *gin.Context) {
	exercises, err := h.workoutService.ListExercises()
	if err != nil {
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to list exercises"))
		return
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(exercises))
}

// WorkoutsPage renders the workouts page
// GET /workouts
func (h *WorkoutHandler) WorkoutsPage(c *gin.Context) {
	userID := middleware.GetUserID(c)
	today := getCurrentDate()

	summary, _ := h.workoutService.GetDailySummary(userID, today)
	exercises, _ := h.workoutService.ListExercises()

	c.HTML(http.StatusOK, "workouts.html", gin.H{
		"summary":   summary,
		"exercises": exercises,
		"today":     today,
		"username":  middleware.GetUsername(c),
	})
}
