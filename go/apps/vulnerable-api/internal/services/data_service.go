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

// ===============================================
// MULTI-HOP TAINT FLOWS (2+ hops)
// Handler (hop 1) -> Service (hop 2) -> SQL Sink
// ===============================================

// ProcessUserQuery processes a user query through multiple hops
// HOP 2: Receives tainted query from handler
// TAINT SINK: SQL injection
func (s *DataService) ProcessUserQuery(query string) ([]map[string]interface{}, error) {
	// TAINT PROPAGATION: Query flows to SQL
	// This is hop 2 - the handler is hop 1
	sqlQuery := fmt.Sprintf("SELECT * FROM users WHERE %s", query)

	// TAINT SINK: SQL injection
	rows, err := s.db.Query(sqlQuery)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	return scanRows(rows)
}

// SearchWithFilters demonstrates multiple tainted parameters
// HOP 2: All parameters are tainted from handler
func (s *DataService) SearchWithFilters(searchTerm, category, sortBy, sortOrder string, limit int) ([]map[string]interface{}, error) {
	// TAINT PROPAGATION: Multiple parameters
	query := fmt.Sprintf(
		"SELECT * FROM items WHERE name LIKE '%%%s%%' AND category = '%s' ORDER BY %s %s LIMIT %d",
		searchTerm, category, sortBy, sortOrder, limit,
	)

	// TAINT SINK: SQL injection (4 injection points)
	rows, err := s.db.Query(query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	return scanRows(rows)
}

// UpdateDynamicField demonstrates column name injection
// HOP 2: Field name and value both tainted
func (s *DataService) UpdateDynamicField(tableName, fieldName, fieldValue, whereClause string) error {
	// TAINT PROPAGATION: All parameters in query
	// This allows table name, column name, and value injection
	query := fmt.Sprintf(
		"UPDATE %s SET %s = '%s' WHERE %s",
		tableName, fieldName, fieldValue, whereClause,
	)

	// TAINT SINK: SQL injection (4 injection points)
	_, err := s.db.Exec(query)
	return err
}

// ===============================================
// 3-HOP FLOWS: Handler -> Service -> Helper
// ===============================================

// ProcessComplexQuery demonstrates 3-hop flow
// HOP 2: Receives from handler, calls helper
func (s *DataService) ProcessComplexQuery(userInput string) ([]map[string]interface{}, error) {
	// HOP 2: Transform input (taint propagates)
	transformedQuery := s.buildQueryFromInput(userInput)

	// HOP 3: Execute query (final sink)
	return s.executeQuery(transformedQuery)
}

// buildQueryFromInput is a helper that propagates taint
// HOP 3 (part 1): Transforms input but taint remains
func (s *DataService) buildQueryFromInput(input string) string {
	// TAINT PROPAGATION: Input flows through transformation
	// Even though we "build" a query, the taint propagates
	return "SELECT * FROM data WHERE " + input
}

// executeQuery is the final sink
// HOP 3 (part 2): Executes the tainted query
func (s *DataService) executeQuery(query string) ([]map[string]interface{}, error) {
	// TAINT SINK: SQL injection
	rows, err := s.db.Query(query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	return scanRows(rows)
}

// ===============================================
// CROSS-FILE FLOWS: Service -> External Package
// ===============================================

// ProcessWithExternalHelper uses an external helper
// HOP 2: Receives from handler, calls cross-file helper
func (s *DataService) ProcessWithExternalHelper(input string) error {
	// Cross-file taint flow: service -> helpers package
	// Even if helper is in different file, taint should track
	processed := ProcessInput(input) // Call to helper in another file

	// TAINT SINK: SQL with processed (still tainted) input
	query := fmt.Sprintf("INSERT INTO logs (message) VALUES ('%s')", processed)
	_, err := s.db.Exec(query)
	return err
}

// ===============================================
// MULTIPLE SINKS FROM SINGLE SOURCE
// ===============================================

// ProcessDataWithMultipleSinks demonstrates one source -> multiple sinks
// HOP 2: Receives single tainted input, flows to multiple sinks
func (s *DataService) ProcessDataWithMultipleSinks(userInput string) error {
	// TAINT SINK 1: SQL injection (logging)
	logQuery := fmt.Sprintf("INSERT INTO audit_log (action) VALUES ('%s')", userInput)
	s.db.Exec(logQuery)

	// TAINT SINK 2: Command injection
	cmd := exec.Command("sh", "-c", "echo "+userInput)
	cmd.Run()

	// TAINT SINK 3: Path traversal (file write)
	filePath := filepath.Join("./data", userInput+".txt")
	os.WriteFile(filePath, []byte("data"), 0644)

	// TAINT SINK 4: SQL injection (main query)
	mainQuery := fmt.Sprintf("SELECT * FROM users WHERE name = '%s'", userInput)
	_, err := s.db.Query(mainQuery)
	return err
}

// ===============================================
// STRUCT FIELD TAINT PROPAGATION
// ===============================================

// ProcessRequest represents a request structure
type ProcessRequest struct {
	UserID    string            `json:"user_id"`
	Query     string            `json:"query"`
	Filters   map[string]string `json:"filters"`
	OutputDir string            `json:"output_dir"`
}

// ProcessStructuredRequest demonstrates taint through struct fields
// HOP 2: Struct fields are tainted
func (s *DataService) ProcessStructuredRequest(req *ProcessRequest) (map[string]interface{}, error) {
	// TAINT from struct field: req.Query
	rows, err := s.db.Query(req.Query) // SINK 1: SQL injection
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	data, _ := scanRows(rows)

	// TAINT from struct field: req.OutputDir
	outputPath := filepath.Join(req.OutputDir, "output.json")
	content, _ := json.Marshal(data)
	os.WriteFile(outputPath, content, 0644) // SINK 2: Path traversal

	// TAINT from struct field: req.Filters (map values)
	for key, value := range req.Filters {
		// SINK 3: SQL injection from map
		filterQuery := fmt.Sprintf("SELECT * FROM data WHERE %s = '%s'", key, value)
		s.db.Query(filterQuery)
	}

	return map[string]interface{}{"status": "processed", "records": len(data)}, nil
}

// ===============================================
// CONDITIONAL TAINT FLOWS
// ===============================================

// ProcessConditionally demonstrates taint through conditionals
// HOP 2: Taint flows through conditional branches
func (s *DataService) ProcessConditionally(userInput, queryType string) ([]map[string]interface{}, error) {
	var query string

	// TAINT PROPAGATION: Both branches are tainted
	switch queryType {
	case "select":
		query = fmt.Sprintf("SELECT * FROM users WHERE name = '%s'", userInput)
	case "delete":
		query = fmt.Sprintf("DELETE FROM users WHERE name = '%s'", userInput)
	default:
		query = fmt.Sprintf("UPDATE users SET active = 0 WHERE name = '%s'", userInput)
	}

	// TAINT SINK: All branches lead to SQL injection
	rows, err := s.db.Query(query)
	if err != nil {
		// Try as exec for non-SELECT queries
		s.db.Exec(query)
		return nil, nil
	}
	defer rows.Close()

	return scanRows(rows)
}

// ===============================================
// LOOP TAINT PROPAGATION
// ===============================================

// ProcessBatch demonstrates taint propagation in loops
// HOP 2: Array of tainted inputs
func (s *DataService) ProcessBatch(inputs []string) ([]error, error) {
	var errors []error

	// TAINT PROPAGATION: Each iteration processes tainted input
	for _, input := range inputs {
		// TAINT SINK: SQL injection for each input
		query := fmt.Sprintf("INSERT INTO batch_data (value) VALUES ('%s')", input)
		_, err := s.db.Exec(query)
		if err != nil {
			errors = append(errors, err)
		}
	}

	return errors, nil
}

// ===============================================
// RETURN VALUE TAINT
// ===============================================

// FetchAndTransform demonstrates taint in return values
// HOP 2: Returns transformed (but still tainted) data
func (s *DataService) FetchAndTransform(userID string) (string, error) {
	// TAINT SINK 1: SQL injection
	query := fmt.Sprintf("SELECT data FROM user_data WHERE user_id = %s", userID)
	var data string
	err := s.db.QueryRow(query).Scan(&data)
	if err != nil {
		return "", err
	}

	// Transform data (taint propagates through return)
	return "transformed_" + data, nil
}

// UseTransformedData demonstrates using tainted return value
// HOP 3: Uses return value from FetchAndTransform
func (s *DataService) UseTransformedData(userID string) error {
	// HOP 2: Get transformed (tainted) data
	data, err := s.FetchAndTransform(userID)
	if err != nil {
		return err
	}

	// HOP 3 SINK: Use tainted return value in new query
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

// ProcessInput is a cross-file helper that propagates taint
// This function would typically be in a separate helpers.go file
func ProcessInput(input string) string {
	// TAINT PROPAGATION: Input flows through with prefix
	return "processed_" + input
}
