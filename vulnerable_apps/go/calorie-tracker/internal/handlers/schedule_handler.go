package handlers

import (
	"errors"
	"net/http"

	"github.com/gin-gonic/gin"

	"github.com/example/calorie-tracker/internal/middleware"
	"github.com/example/calorie-tracker/internal/models"
	"github.com/example/calorie-tracker/internal/services"
)

// ScheduleHandler handles schedule endpoints
type ScheduleHandler struct {
	scheduleService *services.ScheduleService
}

// NewScheduleHandler creates a new schedule handler
func NewScheduleHandler(scheduleService *services.ScheduleService) *ScheduleHandler {
	return &ScheduleHandler{scheduleService: scheduleService}
}

// ListSchedules lists schedules for the current user
// GET /api/schedules
func (h *ScheduleHandler) ListSchedules(c *gin.Context) {
	userID := middleware.GetUserID(c)

	schedules, err := h.scheduleService.ListForUser(userID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to list schedules"))
		return
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(schedules))
}

// CreateSchedule creates a new schedule
// POST /api/schedules
func (h *ScheduleHandler) CreateSchedule(c *gin.Context) {
	userID := middleware.GetUserID(c)

	var req models.CreateScheduleRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, models.NewErrorResponse("Invalid request body"))
		return
	}

	schedule, err := h.scheduleService.Create(userID, req)
	if err != nil {
		if validationErr, ok := err.(*services.ValidationError); ok {
			c.JSON(http.StatusBadRequest, models.NewValidationErrorResponse(validationErr.Errors))
			return
		}
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to create schedule"))
		return
	}

	c.JSON(http.StatusCreated, models.NewSuccessResponse(schedule))
}

// GetSchedule gets a specific schedule with items
// GET /api/schedules/:id
func (h *ScheduleHandler) GetSchedule(c *gin.Context) {
	userID := middleware.GetUserID(c)
	scheduleID := c.Param("id")

	scheduleWithItems, err := h.scheduleService.GetWithItems(scheduleID)
	if err != nil {
		if errors.Is(err, services.ErrScheduleNotFound) {
			c.JSON(http.StatusNotFound, models.NewErrorResponse("Schedule not found"))
			return
		}
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to get schedule"))
		return
	}

	// Verify ownership
	if scheduleWithItems.Schedule.UserID != userID {
		c.JSON(http.StatusForbidden, models.NewErrorResponse("Not your schedule"))
		return
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(scheduleWithItems))
}

// UpdateSchedule updates a schedule
// PUT /api/schedules/:id
func (h *ScheduleHandler) UpdateSchedule(c *gin.Context) {
	userID := middleware.GetUserID(c)
	scheduleID := c.Param("id")

	var req models.UpdateScheduleRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, models.NewErrorResponse("Invalid request body"))
		return
	}

	schedule, err := h.scheduleService.Update(scheduleID, userID, req)
	if err != nil {
		if errors.Is(err, services.ErrScheduleNotFound) {
			c.JSON(http.StatusNotFound, models.NewErrorResponse("Schedule not found"))
			return
		}
		if errors.Is(err, services.ErrNotYourSchedule) {
			c.JSON(http.StatusForbidden, models.NewErrorResponse("Not your schedule"))
			return
		}
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to update schedule"))
		return
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(schedule))
}

// DeleteSchedule deletes a schedule
// DELETE /api/schedules/:id
func (h *ScheduleHandler) DeleteSchedule(c *gin.Context) {
	userID := middleware.GetUserID(c)
	scheduleID := c.Param("id")

	err := h.scheduleService.Delete(scheduleID, userID)
	if err != nil {
		if errors.Is(err, services.ErrScheduleNotFound) {
			c.JSON(http.StatusNotFound, models.NewErrorResponse("Schedule not found"))
			return
		}
		if errors.Is(err, services.ErrNotYourSchedule) {
			c.JSON(http.StatusForbidden, models.NewErrorResponse("Not your schedule"))
			return
		}
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to delete schedule"))
		return
	}

	c.Status(http.StatusNoContent)
}

// ToggleSchedule toggles schedule active status
// POST /api/schedules/:id/toggle
func (h *ScheduleHandler) ToggleSchedule(c *gin.Context) {
	userID := middleware.GetUserID(c)
	scheduleID := c.Param("id")

	schedule, err := h.scheduleService.ToggleActive(scheduleID, userID)
	if err != nil {
		if errors.Is(err, services.ErrScheduleNotFound) {
			c.JSON(http.StatusNotFound, models.NewErrorResponse("Schedule not found"))
			return
		}
		if errors.Is(err, services.ErrNotYourSchedule) {
			c.JSON(http.StatusForbidden, models.NewErrorResponse("Not your schedule"))
			return
		}
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to toggle schedule"))
		return
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(schedule))
}

// AddItem adds an item to a schedule
// POST /api/schedules/:id/items
func (h *ScheduleHandler) AddItem(c *gin.Context) {
	userID := middleware.GetUserID(c)
	scheduleID := c.Param("id")

	// Verify ownership first
	schedule, err := h.scheduleService.FindByID(scheduleID)
	if err != nil {
		if errors.Is(err, services.ErrScheduleNotFound) {
			c.JSON(http.StatusNotFound, models.NewErrorResponse("Schedule not found"))
			return
		}
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to get schedule"))
		return
	}

	if schedule.UserID != userID {
		c.JSON(http.StatusForbidden, models.NewErrorResponse("Not your schedule"))
		return
	}

	var req models.ScheduleItemRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, models.NewErrorResponse("Invalid request body"))
		return
	}

	item, err := h.scheduleService.AddItem(scheduleID, req)
	if err != nil {
		if validationErr, ok := err.(*services.ValidationError); ok {
			c.JSON(http.StatusBadRequest, models.NewValidationErrorResponse(validationErr.Errors))
			return
		}
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to add item"))
		return
	}

	c.JSON(http.StatusCreated, models.NewSuccessResponse(item))
}

// DeleteItem deletes a schedule item
// DELETE /api/schedules/items/:item_id
func (h *ScheduleHandler) DeleteItem(c *gin.Context) {
	userID := middleware.GetUserID(c)
	itemID := c.Param("item_id")

	err := h.scheduleService.DeleteItem(itemID, userID)
	if err != nil {
		if errors.Is(err, services.ErrItemNotFound) {
			c.JSON(http.StatusNotFound, models.NewErrorResponse("Item not found"))
			return
		}
		if errors.Is(err, services.ErrNotYourSchedule) {
			c.JSON(http.StatusForbidden, models.NewErrorResponse("Not your schedule"))
			return
		}
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to delete item"))
		return
	}

	c.Status(http.StatusNoContent)
}

// SchedulePage renders the schedule page
// GET /schedule
func (h *ScheduleHandler) SchedulePage(c *gin.Context) {
	userID := middleware.GetUserID(c)

	schedules, _ := h.scheduleService.ListForUser(userID)

	c.HTML(http.StatusOK, "schedule.html", gin.H{
		"schedules": schedules,
		"username":  middleware.GetUsername(c),
	})
}
