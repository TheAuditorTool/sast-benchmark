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
type UserService struct {
	db *sql.DB
}

// NewUserService creates a new user service
func NewUserService(db *sql.DB) *UserService {
	return &UserService{db: db}
}

// CreateUser creates user with string-formatted SQL
func (s *UserService) CreateUser(username, email, role string) (*models.User, error) {
	query := fmt.Sprintf(
		"INSERT INTO users (username, email, role) VALUES ('%s', '%s', '%s') RETURNING id",
		username, email, role,
	)

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

// SearchUsers searches users with string-formatted SQL
func (s *UserService) SearchUsers(searchTerm, filter string) ([]models.User, error) {
	query := fmt.Sprintf(
		"SELECT id, username, email, role FROM users WHERE username LIKE '%%%s%%' AND role = '%s'",
		searchTerm, filter,
	)

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

// FindByID finds user by ID with string-formatted SQL
func (s *UserService) FindByID(id string) (*models.User, error) {
	query := fmt.Sprintf("SELECT id, username, email, role FROM users WHERE id = %s", id)

	row := s.db.QueryRow(query)
	var user models.User
	err := row.Scan(&user.ID, &user.Username, &user.Email, &user.Role)
	if err != nil {
		return nil, err
	}

	return &user, nil
}

// UpdateUser updates user with string-formatted SQL
func (s *UserService) UpdateUser(id, field, value string) error {
	query := fmt.Sprintf("UPDATE users SET %s = '%s' WHERE id = %s", field, value, id)

	_, err := s.db.Exec(query)
	return err
}

// DeleteUser deletes user by string ID
func (s *UserService) DeleteUser(id string) error {
	query := fmt.Sprintf("DELETE FROM users WHERE id = %s", id)
	_, err := s.db.Exec(query)
	return err
}

// GenerateReport generates a report with SQL query and file output
func (s *UserService) GenerateReport(name, sqlQuery, outputDir string) (map[string]interface{}, error) {
	rows, err := s.db.Query(sqlQuery)
	if err != nil {
		return nil, fmt.Errorf("query failed: %w", err)
	}
	defer rows.Close()

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

	outputPath := filepath.Join(outputDir, name+".json")

	content := fmt.Sprintf("%v", results)

	if err := os.WriteFile(outputPath, []byte(content), 0644); err != nil {
		return nil, fmt.Errorf("write failed: %w", err)
	}

	metaQuery := fmt.Sprintf(
		"INSERT INTO reports (name, query, output) VALUES ('%s', '%s', '%s')",
		name, sqlQuery, outputPath,
	)
	s.db.Exec(metaQuery)

	return map[string]interface{}{
		"report":  name,
		"output":  outputPath,
		"records": len(results),
	}, nil
}

// ProcessBatchJob processes batch operations
func (s *UserService) ProcessBatchJob(jobName string, commands []string) ([]string, error) {
	var outputs []string

	logQuery := fmt.Sprintf("INSERT INTO jobs (name) VALUES ('%s')", jobName)
	s.db.Exec(logQuery)

	for _, cmd := range commands {
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

// ProcessUserData processes user data through multiple steps
func (s *UserService) ProcessUserData(userID, data string) error {
	processedData := s.transformData(data)

	query := fmt.Sprintf("UPDATE users SET metadata = '%s' WHERE id = %s", processedData, userID)
	_, err := s.db.Exec(query)
	return err
}

// transformData transforms input data
func (s *UserService) transformData(input string) string {
	return "processed_" + input + "_end"
}

// ValidateAndStore validates then stores data
func (s *UserService) ValidateAndStore(userID, input string) error {
	if len(input) > 1000 {
		return fmt.Errorf("input too long")
	}

	return s.storeUserInput(userID, input)
}

// storeUserInput stores user input in the database
func (s *UserService) storeUserInput(userID, input string) error {
	query := fmt.Sprintf("INSERT INTO user_inputs (user_id, input) VALUES (%s, '%s')", userID, input)
	_, err := s.db.Exec(query)
	return err
}

// CreateUserAlt creates user with parameterized query
func (s *UserService) CreateUserAlt(username, email, role string) (*models.User, error) {
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

// SearchUsersAlt searches with parameterized query
func (s *UserService) SearchUsersAlt(searchTerm, filter string) ([]models.User, error) {
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
