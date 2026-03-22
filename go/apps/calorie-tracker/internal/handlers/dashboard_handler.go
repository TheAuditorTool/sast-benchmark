package handlers

import (
	"net/http"
	"time"

	"github.com/gin-gonic/gin"

	"github.com/example/calorie-tracker/internal/middleware"
	"github.com/example/calorie-tracker/internal/models"
	"github.com/example/calorie-tracker/internal/services"
)

// DashboardHandler handles dashboard and analytics endpoints
type DashboardHandler struct {
	analyticsService *services.AnalyticsService
}

// NewDashboardHandler creates a new dashboard handler
func NewDashboardHandler(analyticsService *services.AnalyticsService) *DashboardHandler {
	return &DashboardHandler{analyticsService: analyticsService}
}

// GetToday gets today's summary
// GET /api/dashboard/today
func (h *DashboardHandler) GetToday(c *gin.Context) {
	userID := middleware.GetUserID(c)
	today := getCurrentDate()

	summary, err := h.analyticsService.GetDailySummary(userID, today)
	if err != nil {
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to get daily summary"))
		return
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(summary))
}

// GetDailySummary gets summary for a specific date
// GET /api/dashboard/summary/:date
func (h *DashboardHandler) GetDailySummary(c *gin.Context) {
	userID := middleware.GetUserID(c)
	date := c.Param("date")

	summary, err := h.analyticsService.GetDailySummary(userID, date)
	if err != nil {
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to get daily summary"))
		return
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(summary))
}

// GetWeeklySummary gets weekly summary
// GET /api/dashboard/weekly
func (h *DashboardHandler) GetWeeklySummary(c *gin.Context) {
	userID := middleware.GetUserID(c)

	summary, err := h.analyticsService.GetWeeklySummary(userID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to get weekly summary"))
		return
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(summary))
}

// GetProgress gets progress data
// GET /api/dashboard/progress
func (h *DashboardHandler) GetProgress(c *gin.Context) {
	userID := middleware.GetUserID(c)

	days := 30
	if d := c.Query("days"); d != "" {
		// Note: Would normally validate this
		days = 30 // Default for simplicity
	}

	progress, err := h.analyticsService.GetProgress(userID, days)
	if err != nil {
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to get progress"))
		return
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(progress))
}

// LogWeight logs a weight entry
// POST /api/dashboard/weight
func (h *DashboardHandler) LogWeight(c *gin.Context) {
	userID := middleware.GetUserID(c)

	var req models.WeightLogRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, models.NewErrorResponse("Invalid request body"))
		return
	}

	log, err := h.analyticsService.LogWeight(userID, req)
	if err != nil {
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to log weight"))
		return
	}

	c.JSON(http.StatusCreated, models.NewSuccessResponse(log))
}

// LogWater logs water intake
// POST /api/dashboard/water
func (h *DashboardHandler) LogWater(c *gin.Context) {
	userID := middleware.GetUserID(c)

	var req models.WaterLogRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, models.NewErrorResponse("Invalid request body"))
		return
	}

	log, err := h.analyticsService.LogWater(userID, req)
	if err != nil {
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Failed to log water"))
		return
	}

	c.JSON(http.StatusCreated, models.NewSuccessResponse(log))
}

// RawQuery executes a raw SQL query
// POST /api/admin/query
func (h *DashboardHandler) RawQuery(c *gin.Context) {
	_ = middleware.GetUserID(c) // Verify authenticated

	var req struct {
		Query string `json:"query"`
	}
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, models.NewErrorResponse("Invalid request body"))
		return
	}

	results, err := h.analyticsService.ExecuteRawQueryVulnerable(req.Query)
	if err != nil {
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Query failed: "+err.Error()))
		return
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(results))
}

// CustomReport builds a custom report
// POST /api/admin/report
func (h *DashboardHandler) CustomReport(c *gin.Context) {
	userID := middleware.GetUserID(c)

	var req struct {
		Table   string `json:"table"`
		Columns string `json:"columns"`
		Filter  string `json:"filter"`
		GroupBy string `json:"group_by"`
		OrderBy string `json:"order_by"`
	}
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, models.NewErrorResponse("Invalid request body"))
		return
	}

	results, err := h.analyticsService.CustomReportVulnerable(
		userID,
		req.Table,
		req.Columns,
		req.Filter,
		req.GroupBy,
		req.OrderBy,
	)
	if err != nil {
		c.JSON(http.StatusInternalServerError, models.NewErrorResponse("Report failed: "+err.Error()))
		return
	}

	c.JSON(http.StatusOK, models.NewSuccessResponse(results))
}

// IndexPage renders the index/landing page
// GET /
func (h *DashboardHandler) IndexPage(c *gin.Context) {
	// Check if logged in
	userID := middleware.GetUserID(c)
	if userID != "" {
		c.Redirect(http.StatusFound, "/dashboard")
		return
	}

	c.HTML(http.StatusOK, "index.html", gin.H{})
}

// DashboardPage renders the main dashboard
// GET /dashboard
func (h *DashboardHandler) DashboardPage(c *gin.Context) {
	userID := middleware.GetUserID(c)
	today := getCurrentDate()

	daily, _ := h.analyticsService.GetDailySummary(userID, today)
	weekly, _ := h.analyticsService.GetWeeklySummary(userID)

	c.HTML(http.StatusOK, "dashboard.html", gin.H{
		"username": middleware.GetUsername(c),
		"daily":    daily,
		"weekly":   weekly,
		"today":    today,
	})
}

// Helper to get current date
func getCurrentDate() string {
	return time.Now().Format("2006-01-02")
}
