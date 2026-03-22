package handlers

import (
	"errors"
	"net/http"

	"github.com/gin-gonic/gin"

	"github.com/example/calorie-tracker/internal/middleware"
	"github.com/example/calorie-tracker/internal/models"
	"github.com/example/calorie-tracker/internal/services"
)

// MealHandler handles meal endpoints
type MealHandler struct {
	mealService *services.MealService
}

// NewMealHandler creates a new meal handler
func NewMealHandler(mealService *services.MealService) *MealHandler {
	return &MealHandler{mealService: mealService}
}

// ListMeals lists meals for the current user
// GET /api/meals
func (h *MealHandler) ListMeals(c *gin.Context) {
	userID := middleware.GetUserID(c)

	var pagination models.PaginationParams
	if err := c.ShouldBindQuery(&pagination); err != nil {
		pagination = models.PaginationParams{Page: 1, PerPage: 20}
	}

	var dateFilter models.DateRangeFilter
	c.ShouldBindQuery(&dateFilter)

	meals, err := h.mealService.ListForUser(userID, &pagination, &dateFilter)
	if err != nil {
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to list meals"))
		return
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(meals))
}

// CreateMeal creates a new meal
// POST /api/meals
func (h *MealHandler) CreateMeal(c *gin.Context) {
	userID := middleware.GetUserID(c)

	var req models.CreateMealRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, models.NewErrorResponse("Invalid request body"))
		return
	}

	meal, err := h.mealService.Create(userID, req)
	if err != nil {
		if validationErr, ok := err.(*services.ValidationError); ok {
			c.JSON(http.StatusBadRequest, models.NewValidationErrorResponse(validationErr.Errors))
			return
		}
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to create meal"))
		return
	}

	c.JSON(http.StatusCreated, models.NewSuccessResponse(meal))
}

// GetMeal gets a specific meal
// GET /api/meals/:id
func (h *MealHandler) GetMeal(c *gin.Context) {
	userID := middleware.GetUserID(c)
	mealID := c.Param("id")

	meal, err := h.mealService.FindByID(mealID)
	if err != nil {
		if errors.Is(err, services.ErrMealNotFound) {
			c.JSON(http.StatusNotFound, models.NewErrorResponse("Meal not found"))
			return
		}
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to get meal"))
		return
	}

	// Verify ownership
	if meal.UserID != userID {
		c.JSON(http.StatusForbidden, models.NewErrorResponse("Not your meal"))
		return
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(meal))
}

// UpdateMeal updates a meal
// PUT /api/meals/:id
func (h *MealHandler) UpdateMeal(c *gin.Context) {
	userID := middleware.GetUserID(c)
	mealID := c.Param("id")

	var req models.UpdateMealRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, models.NewErrorResponse("Invalid request body"))
		return
	}

	meal, err := h.mealService.Update(mealID, userID, req)
	if err != nil {
		if errors.Is(err, services.ErrMealNotFound) {
			c.JSON(http.StatusNotFound, models.NewErrorResponse("Meal not found"))
			return
		}
		if errors.Is(err, services.ErrNotYourMeal) {
			c.JSON(http.StatusForbidden, models.NewErrorResponse("Not your meal"))
			return
		}
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to update meal"))
		return
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(meal))
}

// DeleteMeal deletes a meal
// DELETE /api/meals/:id
func (h *MealHandler) DeleteMeal(c *gin.Context) {
	userID := middleware.GetUserID(c)
	mealID := c.Param("id")

	err := h.mealService.Delete(mealID, userID)
	if err != nil {
		if errors.Is(err, services.ErrMealNotFound) {
			c.JSON(http.StatusNotFound, models.NewErrorResponse("Meal not found"))
			return
		}
		if errors.Is(err, services.ErrNotYourMeal) {
			c.JSON(http.StatusForbidden, models.NewErrorResponse("Not your meal"))
			return
		}
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to delete meal"))
		return
	}

	c.Status(http.StatusNoContent)
}

// QuickLog quickly logs a meal
// POST /api/meals/quick
func (h *MealHandler) QuickLog(c *gin.Context) {
	userID := middleware.GetUserID(c)

	var entry models.QuickMealEntry
	if err := c.ShouldBindJSON(&entry); err != nil {
		c.JSON(http.StatusBadRequest, models.NewErrorResponse("Invalid request body"))
		return
	}

	meal, err := h.mealService.QuickLog(userID, entry)
	if err != nil {
		if validationErr, ok := err.(*services.ValidationError); ok {
			c.JSON(http.StatusBadRequest, models.NewValidationErrorResponse(validationErr.Errors))
			return
		}
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to log meal"))
		return
	}

	c.JSON(http.StatusCreated, models.NewSuccessResponse(meal))
}

// SearchMeals searches meals by query
// GET /api/meals/search
func (h *MealHandler) SearchMeals(c *gin.Context) {
	userID := middleware.GetUserID(c)
	searchTerm := c.Query("q")

	if searchTerm == "" {
		c.JSON(http.StatusBadRequest, models.NewErrorResponse("Search term required"))
		return
	}

	meals, err := h.mealService.SearchMealsVulnerable(userID, searchTerm)
	if err != nil {
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Search failed"))
		return
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(meals))
}

// GetDailyMeals gets meals for a specific date
// GET /api/meals/daily/:date
func (h *MealHandler) GetDailyMeals(c *gin.Context) {
	userID := middleware.GetUserID(c)
	date := c.Param("date")

	summary, err := h.mealService.GetDailySummary(userID, date)
	if err != nil {
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to get daily meals"))
		return
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(summary))
}

// MealsPage renders the meals page
// GET /meals
func (h *MealHandler) MealsPage(c *gin.Context) {
	userID := middleware.GetUserID(c)
	today := getCurrentDate()

	summary, _ := h.mealService.GetDailySummary(userID, today)

	c.HTML(http.StatusOK, "meals.html", gin.H{
		"summary":  summary,
		"today":    today,
		"username": middleware.GetUsername(c),
	})
}
