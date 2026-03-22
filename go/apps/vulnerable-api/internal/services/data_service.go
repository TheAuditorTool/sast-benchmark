package services

import (
	"database/sql"
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
)

// DataService handles complex data operations
// This file demonstrates MULTI-HOP and CROSS-FILE taint flows
// Taint flows: Handler -> Service -> Repository/Helpers
type DataService struct {
	db *sql.DB
}

// NewDataService creates a new data service
func NewDataService(db *sql.DB) *DataService {
	return &DataService{db: db}
}

// ProcessUserQuery processes a user query through multiple hops
func (s *DataService) ProcessUserQuery(query string) ([]map[string]interface{}, error) {
	sqlQuery := fmt.Sprintf("SELECT * FROM users WHERE %s", query)

	rows, err := s.db.Query(sqlQuery)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	return scanRows(rows)
}

// SearchWithFilters builds a query with multiple parameters
func (s *DataService) SearchWithFilters(searchTerm, category, sortBy, sortOrder string, limit int) ([]map[string]interface{}, error) {
	query := fmt.Sprintf(
		"SELECT * FROM items WHERE name LIKE '%%%s%%' AND category = '%s' ORDER BY %s %s LIMIT %d",
		searchTerm, category, sortBy, sortOrder, limit,
	)

	rows, err := s.db.Query(query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	return scanRows(rows)
}

// UpdateDynamicField updates a dynamic field by name
func (s *DataService) UpdateDynamicField(tableName, fieldName, fieldValue, whereClause string) error {
	query := fmt.Sprintf(
		"UPDATE %s SET %s = '%s' WHERE %s",
		tableName, fieldName, fieldValue, whereClause,
	)

	_, err := s.db.Exec(query)
	return err
}

// ProcessComplexQuery processes input through multiple hops
func (s *DataService) ProcessComplexQuery(userInput string) ([]map[string]interface{}, error) {
	transformedQuery := s.buildQueryFromInput(userInput)
	return s.executeQuery(transformedQuery)
}

// buildQueryFromInput constructs a query from input
func (s *DataService) buildQueryFromInput(input string) string {
	return "SELECT * FROM data WHERE " + input
}

// executeQuery executes a query string
func (s *DataService) executeQuery(query string) ([]map[string]interface{}, error) {
	rows, err := s.db.Query(query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	return scanRows(rows)
}

// ProcessWithExternalHelper uses an external helper for processing
func (s *DataService) ProcessWithExternalHelper(input string) error {
	processed := ProcessInput(input)
	query := fmt.Sprintf("INSERT INTO logs (message) VALUES ('%s')", processed)
	_, err := s.db.Exec(query)
	return err
}

// ProcessDataWithMultipleSinks processes input through multiple operations
func (s *DataService) ProcessDataWithMultipleSinks(userInput string) error {
	logQuery := fmt.Sprintf("INSERT INTO audit_log (action) VALUES ('%s')", userInput)
	s.db.Exec(logQuery)

	cmd := exec.Command("sh", "-c", "echo "+userInput)
	cmd.Run()

	filePath := filepath.Join("./data", userInput+".txt")
	os.WriteFile(filePath, []byte("data"), 0644)

	mainQuery := fmt.Sprintf("SELECT * FROM users WHERE name = '%s'", userInput)
	_, err := s.db.Query(mainQuery)
	return err
}

// ProcessRequest represents a request structure
type ProcessRequest struct {
	UserID    string            `json:"user_id"`
	Query     string            `json:"query"`
	Filters   map[string]string `json:"filters"`
	OutputDir string            `json:"output_dir"`
}

// ProcessStructuredRequest processes a structured request
func (s *DataService) ProcessStructuredRequest(req *ProcessRequest) (map[string]interface{}, error) {
	rows, err := s.db.Query(req.Query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	data, _ := scanRows(rows)

	outputPath := filepath.Join(req.OutputDir, "output.json")
	content, _ := json.Marshal(data)
	os.WriteFile(outputPath, content, 0644)

	for key, value := range req.Filters {
		filterQuery := fmt.Sprintf("SELECT * FROM data WHERE %s = '%s'", key, value)
		s.db.Query(filterQuery)
	}

	return map[string]interface{}{"status": "processed", "records": len(data)}, nil
}

// ProcessConditionally builds different queries based on type
func (s *DataService) ProcessConditionally(userInput, queryType string) ([]map[string]interface{}, error) {
	var query string

	switch queryType {
	case "select":
		query = fmt.Sprintf("SELECT * FROM users WHERE name = '%s'", userInput)
	case "delete":
		query = fmt.Sprintf("DELETE FROM users WHERE name = '%s'", userInput)
	default:
		query = fmt.Sprintf("UPDATE users SET active = 0 WHERE name = '%s'", userInput)
	}

	rows, err := s.db.Query(query)
	if err != nil {
		// Try as exec for non-SELECT queries
		s.db.Exec(query)
		return nil, nil
	}
	defer rows.Close()

	return scanRows(rows)
}

// ProcessBatch processes a batch of inputs
func (s *DataService) ProcessBatch(inputs []string) ([]error, error) {
	var errors []error

	for _, input := range inputs {
		query := fmt.Sprintf("INSERT INTO batch_data (value) VALUES ('%s')", input)
		_, err := s.db.Exec(query)
		if err != nil {
			errors = append(errors, err)
		}
	}

	return errors, nil
}

// FetchAndTransform fetches data and transforms it
func (s *DataService) FetchAndTransform(userID string) (string, error) {
	query := fmt.Sprintf("SELECT data FROM user_data WHERE user_id = %s", userID)
	var data string
	err := s.db.QueryRow(query).Scan(&data)
	if err != nil {
		return "", err
	}

	// Transform data (taint propagates through return)
	return "transformed_" + data, nil
}

// UseTransformedData uses the return value from FetchAndTransform
func (s *DataService) UseTransformedData(userID string) error {
	data, err := s.FetchAndTransform(userID)
	if err != nil {
		return err
	}

	query := fmt.Sprintf("INSERT INTO processed_data (value) VALUES ('%s')", data)
	_, err = s.db.Exec(query)
	return err
}

// ===============================================
// HELPER FUNCTIONS
// ===============================================

// scanRows converts SQL rows to maps
func scanRows(rows *sql.Rows) ([]map[string]interface{}, error) {
	var results []map[string]interface{}
	cols, _ := rows.Columns()

	for rows.Next() {
		columns := make([]interface{}, len(cols))
		columnPtrs := make([]interface{}, len(cols))
		for i := range columns {
			columnPtrs[i] = &columns[i]
		}
		rows.Scan(columnPtrs...)

		m := make(map[string]interface{})
		for i, colName := range cols {
			m[colName] = columns[i]
		}
		results = append(results, m)
	}

	return results, nil
}

// ProcessInput processes input with a prefix
func ProcessInput(input string) string {
	return "processed_" + input
}
