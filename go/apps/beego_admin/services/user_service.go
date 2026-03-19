package services

import (
	"database/sql"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"

	"github.com/theauditor/beego_admin/models"
)

// UserService handles user business logic
// This layer demonstrates MULTI-HOP taint propagation
// Taint flows: Controller -> Service -> (SQL/Command/File)
type UserService struct {
	db *sql.DB
}

// NewUserService creates a new user service
func NewUserService(db *sql.DB) *UserService {
	return &UserService{db: db}
}

// ===============================================
// MULTI-HOP TAINT FLOWS: Service -> SQL
// ===============================================

// CreateUserVulnerable creates user with vulnerable SQL
// TAINT HOP 2: Receives tainted input from controller
// TAINT SINK: SQL injection
func (s *UserService) CreateUserVulnerable(username, email, role string) (*models.User, error) {
	// TAINT PROPAGATION: Parameters flow to SQL
	query := fmt.Sprintf(
		"INSERT INTO users (username, email, role) VALUES ('%s', '%s', '%s') RETURNING id",
		username, email, role,
	)

	// TAINT SINK: SQL injection (HOP 2)
	var id int64
	err := s.db.QueryRow(query).Scan(&id)
	if err != nil {
		return nil, err
	}

	return &models.User{
		ID:       id,
		Username: username,
		Email:    email,
		Role:     role,
	}, nil
}

// SearchUsersVulnerable searches users with vulnerable SQL
// TAINT HOP 2: Receives searchTerm and filter from controller
func (s *UserService) SearchUsersVulnerable(searchTerm, filter string) ([]models.User, error) {
	// TAINT PROPAGATION: Both parameters tainted
	query := fmt.Sprintf(
		"SELECT id, username, email, role FROM users WHERE username LIKE '%%%s%%' AND role = '%s'",
		searchTerm, filter,
	)

	// TAINT SINK: SQL injection
	rows, err := s.db.Query(query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var users []models.User
	for rows.Next() {
		var user models.User
		if err := rows.Scan(&user.ID, &user.Username, &user.Email, &user.Role); err != nil {
			continue
		}
		users = append(users, user)
	}

	return users, nil
}

// FindByIDVulnerable finds user by ID
// TAINT HOP 2: ID is tainted
func (s *UserService) FindByIDVulnerable(id string) (*models.User, error) {
	// TAINT PROPAGATION -> SINK
	query := fmt.Sprintf("SELECT id, username, email, role FROM users WHERE id = %s", id)

	row := s.db.QueryRow(query)
	var user models.User
	err := row.Scan(&user.ID, &user.Username, &user.Email, &user.Role)
	if err != nil {
		return nil, err
	}

	return &user, nil
}

// UpdateUserVulnerable updates user with vulnerable SQL
// TAINT HOP 2: All parameters tainted
func (s *UserService) UpdateUserVulnerable(id, field, value string) error {
	// TAINT PROPAGATION: All three parameters flow to SQL
	// Note: 'field' allows column name injection too
	query := fmt.Sprintf("UPDATE users SET %s = '%s' WHERE id = %s", field, value, id)

	// TAINT SINK: SQL injection
	_, err := s.db.Exec(query)
	return err
}

// DeleteUserVulnerable deletes user
// TAINT HOP 2: ID is tainted
func (s *UserService) DeleteUserVulnerable(id string) error {
	query := fmt.Sprintf("DELETE FROM users WHERE id = %s", id)
	_, err := s.db.Exec(query)
	return err
}

// ===============================================
// COMPLEX MULTI-HOP: Service -> Multiple Sinks
// ===============================================

// GenerateReport generates a report (SQL + File sinks)
// TAINT HOP 2: Receives name, query, outputDir from controller
// This method has MULTIPLE SINKS from single input
func (s *UserService) GenerateReport(name, sqlQuery, outputDir string) (map[string]interface{}, error) {
	// TAINT SINK 1: SQL injection via sqlQuery parameter
	// User can execute arbitrary SQL
	rows, err := s.db.Query(sqlQuery)
	if err != nil {
		return nil, fmt.Errorf("query failed: %w", err)
	}
	defer rows.Close()

	// Collect results
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

	// TAINT SINK 2: Path traversal via outputDir
	// User can write to arbitrary location
	outputPath := filepath.Join(outputDir, name+".json")

	// Convert results to JSON
	content := fmt.Sprintf("%v", results)

	// TAINT SINK: Arbitrary file write
	if err := os.WriteFile(outputPath, []byte(content), 0644); err != nil {
		return nil, fmt.Errorf("write failed: %w", err)
	}

	// Store report metadata
	metaQuery := fmt.Sprintf(
		"INSERT INTO reports (name, query, output) VALUES ('%s', '%s', '%s')",
		name, sqlQuery, outputPath,
	)
	// TAINT SINK 3: Another SQL injection storing the report
	s.db.Exec(metaQuery)

	return map[string]interface{}{
		"report":  name,
		"output":  outputPath,
		"records": len(results),
	}, nil
}

// ProcessBatchJob processes batch operations
// TAINT HOP 2: Receives commands from controller
// Multiple command injection sinks
func (s *UserService) ProcessBatchJob(jobName string, commands []string) ([]string, error) {
	var outputs []string

	// TAINT SINK 1: SQL to log job start
	logQuery := fmt.Sprintf("INSERT INTO jobs (name) VALUES ('%s')", jobName)
	s.db.Exec(logQuery)

	for _, cmd := range commands {
		// TAINT SINK 2: Command injection for each command
		command := exec.Command("sh", "-c", cmd)
		output, err := command.CombinedOutput()
		if err != nil {
			outputs = append(outputs, fmt.Sprintf("ERROR: %s", err.Error()))
			continue
		}
		outputs = append(outputs, string(output))
	}

	return outputs, nil
}

// ===============================================
// 3+ HOP FLOWS: Controller -> Service -> Helper
// ===============================================

// ProcessUserData demonstrates 3-hop flow
// TAINT HOP 2: Receives data from controller
func (s *UserService) ProcessUserData(userID, data string) error {
	// TAINT HOP 3: Pass to helper function
	processedData := s.transformData(data)

	// TAINT SINK: SQL with processed (but still tainted) data
	query := fmt.Sprintf("UPDATE users SET metadata = '%s' WHERE id = %s", processedData, userID)
	_, err := s.db.Exec(query)
	return err
}

// transformData is a helper that propagates taint
// TAINT HOP 3: Receives tainted data, returns tainted data
func (s *UserService) transformData(input string) string {
	// Taint propagates through transformation
	// Even though we "process" it, the taint remains
	return "processed_" + input + "_end"
}

// ValidateAndStore validates then stores data
// TAINT HOP 2: Demonstrates validation that doesn't sanitize
func (s *UserService) ValidateAndStore(userID, input string) error {
	// "Validation" that doesn't actually sanitize
	if len(input) > 1000 {
		return fmt.Errorf("input too long")
	}

	// Taint still flows through despite "validation"
	return s.storeUserInput(userID, input)
}

// storeUserInput stores user input
// TAINT HOP 3: Final sink
func (s *UserService) storeUserInput(userID, input string) error {
	query := fmt.Sprintf("INSERT INTO user_inputs (user_id, input) VALUES (%s, '%s')", userID, input)
	_, err := s.db.Exec(query)
	return err
}

// ===============================================
// SECURE EXAMPLES (for comparison)
// ===============================================

// CreateUserSecure creates user with parameterized query
func (s *UserService) CreateUserSecure(username, email, role string) (*models.User, error) {
	query := "INSERT INTO users (username, email, role) VALUES (?, ?, ?) RETURNING id"

	var id int64
	err := s.db.QueryRow(query, username, email, role).Scan(&id)
	if err != nil {
		return nil, err
	}

	return &models.User{
		ID:       id,
		Username: username,
		Email:    email,
		Role:     role,
	}, nil
}

// SearchUsersSecure searches with parameterized query
func (s *UserService) SearchUsersSecure(searchTerm, filter string) ([]models.User, error) {
	query := "SELECT id, username, email, role FROM users WHERE username LIKE ? AND role = ?"

	rows, err := s.db.Query(query, "%"+searchTerm+"%", filter)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var users []models.User
	for rows.Next() {
		var user models.User
		if err := rows.Scan(&user.ID, &user.Username, &user.Email, &user.Role); err != nil {
			continue
		}
		users = append(users, user)
	}

	return users, nil
}
