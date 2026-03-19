package handlers

import (
	"errors"
	"net/http"

	"github.com/gin-gonic/gin"

	"github.com/example/calorie-tracker/internal/models"
	"github.com/example/calorie-tracker/internal/services"
)

// FoodHandler handles food database endpoints
type FoodHandler struct {
	foodService *services.FoodService
}

// NewFoodHandler creates a new food handler
func NewFoodHandler(foodService *services.FoodService) *FoodHandler {
	return &FoodHandler{foodService: foodService}
}

// ListFoods lists all foods
// GET /api/foods
func (h *FoodHandler) ListFoods(c *gin.Context) {
	category := c.Query("category")

	var foods []models.Food
	var err error

	if category != "" {
		foods, err = h.foodService.ListByCategory(category)
	} else {
		foods, err = h.foodService.ListAll()
	}

	if err != nil {
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to list foods"))
		return
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(foods))
}

// CreateFood creates a new custom food
// POST /api/foods
// TAINT SOURCE: JSON body -> service -> repository -> database
func (h *FoodHandler) CreateFood(c *gin.Context) {
	var req models.CreateFoodRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, models.NewErrorResponse("Invalid request body"))
		return
	}

	food, err := h.foodService.Create(req)
	if err != nil {
		if validationErr, ok := err.(*services.ValidationError); ok {
			c.JSON(http.StatusBadRequest, models.NewValidationErrorResponse(validationErr.Errors))
			return
		}
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to create food"))
		return
	}

	c.JSON(http.StatusCreated, models.NewSuccessResponse(food))
}

// SearchFoods searches foods - VULNERABLE
// GET /api/foods/search
// TAINT SOURCE: Query param "q" -> service -> repository -> SQL injection
func (h *FoodHandler) SearchFoods(c *gin.Context) {
	searchTerm := c.Query("q")

	if searchTerm == "" {
		c.JSON(http.StatusBadRequest, models.NewErrorResponse("Search term required"))
		return
	}

	// VULNERABLE: searchTerm flows to SQL without proper sanitization
	foods, err := h.foodService.SearchVulnerable(searchTerm)
	if err != nil {
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Search failed"))
		return
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(foods))
}

// GetFood gets a specific food
// GET /api/foods/:id
func (h *FoodHandler) GetFood(c *gin.Context) {
	foodID := c.Param("id")

	food, err := h.foodService.FindByID(foodID)
	if err != nil {
		if errors.Is(err, services.ErrFoodNotFound) {
			c.JSON(http.StatusNotFound, models.NewErrorResponse("Food not found"))
			return
		}
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to get food"))
		return
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(food))
}
