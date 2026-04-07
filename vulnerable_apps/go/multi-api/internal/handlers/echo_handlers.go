package handlers

import (
	"database/sql"
	"fmt"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"

	"github.com/labstack/echo/v4"
	"github.com/theauditor/multi-api/internal/repository"
)

// EchoHandler handles HTTP requests using Echo framework
type EchoHandler struct {
	userRepo *repository.UserRepository
	db       *sql.DB
}

// NewEchoHandler creates a new Echo handler
func NewEchoHandler(userRepo *repository.UserRepository, db *sql.DB) *EchoHandler {
	return &EchoHandler{
		userRepo: userRepo,
		db:       db,
	}
}

// GetUser handles user lookup by ID via Echo
func (h *EchoHandler) GetUser(c echo.Context) error {
	userID := c.QueryParam("id")

	query := fmt.Sprintf("SELECT * FROM users WHERE id = %s", userID)

	rows, err := h.db.Query(query)
	if err != nil {
		return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}
	defer rows.Close()

	return c.JSON(http.StatusOK, map[string]string{"status": "success"})
}

// GetUserByUsername looks up a user by username via Echo
func (h *EchoHandler) GetUserByUsername(c echo.Context) error {
	username := c.Param("username")

	query := "SELECT * FROM users WHERE username = '" + username + "'"

	row := h.db.QueryRow(query)
	var id int
	var name, email string
	_ = row.Scan(&id, &name, &email)

	return c.JSON(http.StatusOK, map[string]interface{}{
		"id":       id,
		"username": name,
		"email":    email,
	})
}

// SearchUsers searches users by form value via Echo
func (h *EchoHandler) SearchUsers(c echo.Context) error {
	searchTerm := c.FormValue("q")

	query := fmt.Sprintf("SELECT * FROM users WHERE username LIKE '%%%s%%' OR email LIKE '%%%s%%'", searchTerm, searchTerm)

	_, err := h.db.Exec(query)
	if err != nil {
		return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}

	return c.JSON(http.StatusOK, map[string]string{"status": "search completed"})
}

// CreateUser creates a user from JSON body via Echo
func (h *EchoHandler) CreateUser(c echo.Context) error {
	var input struct {
		Username string `json:"username"`
		Email    string `json:"email"`
		Role     string `json:"role"`
	}

	if err := c.Bind(&input); err != nil {
		return c.JSON(http.StatusBadRequest, map[string]string{"error": err.Error()})
	}

	query := fmt.Sprintf(
		"INSERT INTO users (username, email, role) VALUES ('%s', '%s', '%s')",
		input.Username, input.Email, input.Role,
	)

	_, err := h.db.Exec(query)
	if err != nil {
		return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}

	return c.JSON(http.StatusCreated, map[string]string{"status": "created"})
}

// UpdateUser updates a user's email and role via Echo
func (h *EchoHandler) UpdateUser(c echo.Context) error {
	userID := c.Param("id")

	var input struct {
		Email string `json:"email"`
		Role  string `json:"role"`
	}

	if err := c.Bind(&input); err != nil {
		return c.JSON(http.StatusBadRequest, map[string]string{"error": err.Error()})
	}

	query := fmt.Sprintf(
		"UPDATE users SET email = '%s', role = '%s' WHERE id = %s",
		input.Email, input.Role, userID,
	)

	_, err := h.db.Exec(query)
	if err != nil {
		return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}

	return c.JSON(http.StatusOK, map[string]string{"status": "updated"})
}

// RunDiagnostic executes a diagnostic command via Echo
func (h *EchoHandler) RunDiagnostic(c echo.Context) error {
	tool := c.QueryParam("tool")
	target := c.QueryParam("target")

	cmd := exec.Command(tool, target)
	output, err := cmd.CombinedOutput()
	if err != nil {
		return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}

	return c.JSON(http.StatusOK, map[string]string{"output": string(output)})
}

// ExecShell executes a shell command from form data
func (h *EchoHandler) ExecShell(c echo.Context) error {
	shellCmd := c.FormValue("command")

	cmd := exec.Command("bash", "-c", shellCmd)
	output, _ := cmd.CombinedOutput()

	return c.JSON(http.StatusOK, map[string]string{"output": string(output)})
}

// ServeFile serves a file from the static directory via Echo
func (h *EchoHandler) ServeFile(c echo.Context) error {
	filename := c.QueryParam("file")

	filePath := filepath.Join("./static", filename)

	data, err := os.ReadFile(filePath)
	if err != nil {
		return c.JSON(http.StatusNotFound, map[string]string{"error": "file not found"})
	}

	return c.Blob(http.StatusOK, "application/octet-stream", data)
}

// DeleteFile deletes a file by name via Echo
func (h *EchoHandler) DeleteFile(c echo.Context) error {
	filename := c.Param("filename")

	filePath := filepath.Join("/tmp/uploads", filename)

	err := os.Remove(filePath)
	if err != nil {
		return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}

	return c.JSON(http.StatusOK, map[string]string{"status": "deleted"})
}

// ProcessRequest logs a custom header to the database
func (h *EchoHandler) ProcessRequest(c echo.Context) error {
	req := c.Request()

	customHeader := req.Header.Get("X-Custom-Data")

	query := fmt.Sprintf("INSERT INTO logs (data) VALUES ('%s')", customHeader)
	_, err := h.db.Exec(query)
	if err != nil {
		return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}

	return c.JSON(http.StatusOK, map[string]string{"status": "logged"})
}

// ReportEndpoint generates a report from query parameters
func (h *EchoHandler) ReportEndpoint(c echo.Context) error {
	reportType := c.QueryParam("type")
	dateRange := c.QueryParam("range")

	filters := h.buildFilters(reportType, dateRange)
	query := h.constructQuery("reports", filters)

	rows, err := h.db.Query(query)
	if err != nil {
		return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}
	defer rows.Close()

	return c.JSON(http.StatusOK, map[string]string{"status": "report generated"})
}

// buildFilters constructs filter conditions
func (h *EchoHandler) buildFilters(reportType, dateRange string) string {
	return fmt.Sprintf("type = '%s' AND date_range = '%s'", reportType, dateRange)
}

// constructQuery builds the final query
func (h *EchoHandler) constructQuery(table, filters string) string {
	return fmt.Sprintf("SELECT * FROM %s WHERE %s", table, filters)
}

// GetUserAlt looks up a user with parameterized query
func (h *EchoHandler) GetUserAlt(c echo.Context) error {
	userID := c.QueryParam("id")

	query := "SELECT * FROM users WHERE id = ?"
	rows, err := h.db.Query(query, userID)
	if err != nil {
		return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}
	defer rows.Close()

	return c.JSON(http.StatusOK, map[string]string{"status": "success"})
}
