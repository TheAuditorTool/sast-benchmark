package services

import (
	"time"

	"github.com/example/calorie-tracker/internal/models"
	"github.com/example/calorie-tracker/internal/repository"
)

// AnalyticsService handles analytics business logic
type AnalyticsService struct {
	analyticsRepo *repository.AnalyticsRepository
	mealRepo      *repository.MealRepository
	workoutRepo   *repository.WorkoutRepository
	userRepo      *repository.UserRepository
}

// NewAnalyticsService creates a new analytics service
func NewAnalyticsService(
	analyticsRepo *repository.AnalyticsRepository,
	mealRepo *repository.MealRepository,
	workoutRepo *repository.WorkoutRepository,
	userRepo *repository.UserRepository,
) *AnalyticsService {
	return &AnalyticsService{
		analyticsRepo: analyticsRepo,
		mealRepo:      mealRepo,
		workoutRepo:   workoutRepo,
		userRepo:      userRepo,
	}
}

// GetDailySummary gets comprehensive daily summary
// TAINT PROPAGATION: Multiple repository calls aggregate data
func (s *AnalyticsService) GetDailySummary(userID, date string) (*models.DailySummary, error) {
	// Get user for calorie goal
	user, err := s.userRepo.FindByID(userID)
	if err != nil {
		return nil, err
	}

	// Get meal summary
	mealSummary, err := s.mealRepo.GetDailySummary(userID, date)
	if err != nil {
		return nil, err
	}

	// Get workout summary
	workoutSummary, err := s.workoutRepo.GetDailySummary(userID, date)
	if err != nil {
		return nil, err
	}

	// Get daily log for water
	dailyLog, _ := s.analyticsRepo.GetOrCreateDailyLog(userID, date)

	summary := &models.DailySummary{
		Date:             date,
		CaloriesConsumed: mealSummary.TotalCalories,
		CaloriesBurned:   workoutSummary.TotalCaloriesBurned,
		NetCalories:      mealSummary.TotalCalories - workoutSummary.TotalCaloriesBurned,
		CalorieGoal:      user.CalorieGoal,
		ProteinGrams:     mealSummary.TotalProtein,
		CarbsGrams:       mealSummary.TotalCarbs,
		FatGrams:         mealSummary.TotalFat,
		WaterMl:          dailyLog.WaterMl,
		MealCount:        mealSummary.MealCount,
		WorkoutCount:     workoutSummary.TotalWorkouts,
	}

	return summary, nil
}

// GetWeeklySummary gets weekly summary
func (s *AnalyticsService) GetWeeklySummary(userID string) (*models.WeeklySummary, error) {
	now := time.Now()
	endDate := now
	startDate := now.AddDate(0, 0, -7)

	// Get meal calories for the week
	mealCalories, err := s.mealRepo.GetCaloriesForDateRange(userID, startDate, endDate)
	if err != nil {
		return nil, err
	}

	// Get workout calories for the week
	workoutCalories, err := s.workoutRepo.GetCaloriesBurnedForDateRange(userID, startDate, endDate)
	if err != nil {
		return nil, err
	}

	// Calculate averages and totals
	totalCalories := 0
	totalBurned := 0
	totalProtein := 0.0
	totalCarbs := 0.0
	totalFat := 0.0
	days := 0

	for d := startDate; d.Before(endDate); d = d.AddDate(0, 0, 1) {
		dateStr := d.Format("2006-01-02")
		if cal, ok := mealCalories[dateStr]; ok {
			totalCalories += cal
			days++
		}
		if burned, ok := workoutCalories[dateStr]; ok {
			totalBurned += burned
		}
	}

	avgCalories := 0.0
	if days > 0 {
		avgCalories = float64(totalCalories) / float64(days)
	}

	summary := &models.WeeklySummary{
		StartDate:           startDate.Format("2006-01-02"),
		EndDate:             endDate.Format("2006-01-02"),
		AvgCalories:         avgCalories,
		TotalCaloriesBurned: totalBurned,
		TotalWorkouts:       len(workoutCalories),
		AvgProtein:          totalProtein / float64(max(days, 1)),
		AvgCarbs:            totalCarbs / float64(max(days, 1)),
		AvgFat:              totalFat / float64(max(days, 1)),
	}

	return summary, nil
}

// GetProgress gets progress data for charts
func (s *AnalyticsService) GetProgress(userID string, days int) (*models.ProgressData, error) {
	now := time.Now()
	endDate := now
	startDate := now.AddDate(0, 0, -days)

	// Get meal calories
	mealCalories, err := s.mealRepo.GetCaloriesForDateRange(userID, startDate, endDate)
	if err != nil {
		return nil, err
	}

	// Get workout calories
	workoutCalories, err := s.workoutRepo.GetCaloriesBurnedForDateRange(userID, startDate, endDate)
	if err != nil {
		return nil, err
	}

	// Get weight logs
	weightLogs, err := s.analyticsRepo.GetWeightLogsInRange(userID, startDate, endDate)
	if err != nil {
		return nil, err
	}

	// Build progress data
	var dates []string
	var calories []int
	var burned []int
	var weights []float64
	var weightDates []string

	totalCalories := 0
	for d := startDate; d.Before(endDate); d = d.AddDate(0, 0, 1) {
		dateStr := d.Format("2006-01-02")
		dates = append(dates, dateStr)

		cal := mealCalories[dateStr]
		calories = append(calories, cal)
		totalCalories += cal

		burnedCal := workoutCalories[dateStr]
		burned = append(burned, burnedCal)
	}

	for _, w := range weightLogs {
		weights = append(weights, w.WeightKg)
		weightDates = append(weightDates, w.LoggedAt.Format("2006-01-02"))
	}

	var startWeight, currentWeight, weightChange float64
	if len(weightLogs) > 0 {
		startWeight = weightLogs[0].WeightKg
		currentWeight = weightLogs[len(weightLogs)-1].WeightKg
		weightChange = currentWeight - startWeight
	}

	avgDailyCalories := 0.0
	if len(dates) > 0 {
		avgDailyCalories = float64(totalCalories) / float64(len(dates))
	}

	progress := &models.ProgressData{
		Dates:            dates,
		Calories:         calories,
		CaloriesBurned:   burned,
		Weights:          weights,
		WeightDates:      weightDates,
		StartWeight:      startWeight,
		CurrentWeight:    currentWeight,
		WeightChange:     weightChange,
		AvgDailyCalories: avgDailyCalories,
	}

	return progress, nil
}

// LogWeight logs a weight entry
func (s *AnalyticsService) LogWeight(userID string, req models.WeightLogRequest) (*models.WeightLog, error) {
	log := &models.WeightLog{
		UserID:   userID,
		WeightKg: req.WeightKg,
		LoggedAt: time.Now(),
		Notes:    req.Notes,
	}

	if err := s.analyticsRepo.CreateWeightLog(log); err != nil {
		return nil, err
	}

	return log, nil
}

// LogWater logs water intake
func (s *AnalyticsService) LogWater(userID string, req models.WaterLogRequest) (*models.DailyLog, error) {
	date := time.Now().Format("2006-01-02")
	return s.analyticsRepo.UpdateWaterLog(userID, date, req.AmountMl)
}

// ExecuteRawQueryVulnerable executes raw SQL - VULNERABLE
// TAINT SINK: Direct SQL execution from user input
func (s *AnalyticsService) ExecuteRawQueryVulnerable(query string) ([]map[string]interface{}, error) {
	// VULNERABLE: Query flows directly to database
	return s.analyticsRepo.ExecuteRawQuery(query)
}

// CustomReportVulnerable builds a custom report - VULNERABLE
// TAINT FLOW: Multiple user inputs flow through helpers to SQL
func (s *AnalyticsService) CustomReportVulnerable(
	userID string,
	table string,
	columns string,
	filter string,
	groupBy string,
	orderBy string,
) ([]map[string]interface{}, error) {
	// Build WHERE clause with user_id
	whereClause := "user_id = '" + userID + "'"
	if filter != "" {
		whereClause += " AND " + filter
	}

	// VULNERABLE: All parameters flow to SQL
	return s.analyticsRepo.CustomReportVulnerable(table, columns, whereClause, groupBy, orderBy)
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
