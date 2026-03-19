package repository

import (
	"fmt"
	"time"

	"github.com/example/calorie-tracker/internal/models"
	"gorm.io/gorm"
)

// AnalyticsRepository handles analytics database operations
type AnalyticsRepository struct {
	db *gorm.DB
}

// NewAnalyticsRepository creates a new analytics repository
func NewAnalyticsRepository(db *gorm.DB) *AnalyticsRepository {
	return &AnalyticsRepository{db: db}
}

// GetOrCreateDailyLog gets or creates a daily log
func (r *AnalyticsRepository) GetOrCreateDailyLog(userID, date string) (*models.DailyLog, error) {
	var log models.DailyLog
	err := r.db.Where("user_id = ? AND date = ?", userID, date).First(&log).Error
	if err == gorm.ErrRecordNotFound {
		log = models.DailyLog{
			UserID: userID,
			Date:   date,
		}
		if err := r.db.Create(&log).Error; err != nil {
			return nil, err
		}
	} else if err != nil {
		return nil, err
	}
	return &log, nil
}

// UpdateWaterLog updates the water intake for a day
func (r *AnalyticsRepository) UpdateWaterLog(userID, date string, amountMl int) (*models.DailyLog, error) {
	log, err := r.GetOrCreateDailyLog(userID, date)
	if err != nil {
		return nil, err
	}

	log.WaterMl += amountMl
	if err := r.db.Save(log).Error; err != nil {
		return nil, err
	}

	return log, nil
}

// CreateWeightLog creates a weight log entry
func (r *AnalyticsRepository) CreateWeightLog(log *models.WeightLog) error {
	return r.db.Create(log).Error
}

// GetWeightLogs gets weight logs for a user
func (r *AnalyticsRepository) GetWeightLogs(userID string, limit int) ([]models.WeightLog, error) {
	var logs []models.WeightLog
	err := r.db.Where("user_id = ?", userID).
		Order("logged_at DESC").
		Limit(limit).
		Find(&logs).Error
	return logs, err
}

// GetLatestWeight gets the most recent weight
func (r *AnalyticsRepository) GetLatestWeight(userID string) (*models.WeightLog, error) {
	var log models.WeightLog
	err := r.db.Where("user_id = ?", userID).
		Order("logged_at DESC").
		First(&log).Error
	if err != nil {
		return nil, err
	}
	return &log, nil
}

// GetWeightLogsInRange gets weight logs within a date range
func (r *AnalyticsRepository) GetWeightLogsInRange(userID string, startDate, endDate time.Time) ([]models.WeightLog, error) {
	var logs []models.WeightLog
	err := r.db.Where("user_id = ? AND logged_at BETWEEN ? AND ?", userID, startDate, endDate).
		Order("logged_at ASC").
		Find(&logs).Error
	return logs, err
}

// CustomReportVulnerable builds a custom report with SQL injection
// TAINT SINK: Multiple user inputs flow to SQL
func (r *AnalyticsRepository) CustomReportVulnerable(
	table string,
	columns string,
	whereClause string,
	groupBy string,
	orderBy string,
) ([]map[string]interface{}, error) {
	// Build query with user-controlled inputs - VULNERABLE
	// TAINT FLOW: table, columns, whereClause, groupBy, orderBy -> SQL

	columnClause := buildColumnClause(columns)
	groupClause := buildGroupClause(groupBy)
	orderClause := buildOrderClause(orderBy)

	// VULNERABLE: Direct string interpolation
	query := fmt.Sprintf(
		"SELECT %s FROM %s WHERE %s %s %s",
		columnClause, table, whereClause, groupClause, orderClause,
	)

	var results []map[string]interface{}
	rows, err := r.db.Raw(query).Rows()
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	columns_arr, _ := rows.Columns()
	for rows.Next() {
		values := make([]interface{}, len(columns_arr))
		valuePtrs := make([]interface{}, len(columns_arr))
		for i := range values {
			valuePtrs[i] = &values[i]
		}
		rows.Scan(valuePtrs...)
		row := make(map[string]interface{})
		for i, col := range columns_arr {
			row[col] = values[i]
		}
		results = append(results, row)
	}
	return results, nil
}

// Helper functions for building SQL clauses - TAINT PROPAGATION
func buildColumnClause(columns string) string {
	if columns == "" {
		return "*"
	}
	// TAINT PROPAGATION: User input passes through
	return columns
}

func buildGroupClause(groupBy string) string {
	if groupBy == "" {
		return ""
	}
	// TAINT PROPAGATION: User input passes through
	return "GROUP BY " + groupBy
}

func buildOrderClause(orderBy string) string {
	if orderBy == "" {
		return ""
	}
	// TAINT PROPAGATION: User input passes through
	return "ORDER BY " + orderBy
}

// ExecuteRawQuery executes a raw SQL query - VULNERABLE
// TAINT SINK: Direct SQL execution
func (r *AnalyticsRepository) ExecuteRawQuery(query string) ([]map[string]interface{}, error) {
	var results []map[string]interface{}
	rows, err := r.db.Raw(query).Rows()
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	columns, _ := rows.Columns()
	for rows.Next() {
		values := make([]interface{}, len(columns))
		valuePtrs := make([]interface{}, len(columns))
		for i := range values {
			valuePtrs[i] = &values[i]
		}
		rows.Scan(valuePtrs...)
		row := make(map[string]interface{})
		for i, col := range columns {
			row[col] = values[i]
		}
		results = append(results, row)
	}
	return results, nil
}
