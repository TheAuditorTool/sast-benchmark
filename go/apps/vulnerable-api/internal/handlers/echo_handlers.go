package handlers

import (
	"database/sql"
	"fmt"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"

	"github.com/labstack/echo/v4"
	"github.com/theauditor/vulnerable-api/internal/repository"
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

// ===============================================
// TAINT SOURCE: c.QueryParam, c.Param, c.FormValue
// TAINT SINK: SQL injection
// ===============================================

// GetUser - VULNERABLE: SQL injection via query parameter
// Taint flow: c.QueryParam("id") -> fmt.Sprintf -> db.Query
func (h *EchoHandler) GetUser(c echo.Context) error {
	// TAINT SOURCE: Query parameter from Echo context
	userID := c.QueryParam("id")

	// TAINT PROPAGATION: User input flows into SQL query
	query := fmt.Sprintf("SELECT * FROM users WHERE id = %s", userID)

	// TAINT SINK: SQL injection
	rows, err := h.db.Query(query)
	if err != nil {
		return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}
	defer rows.Close()

	return c.JSON(http.StatusOK, map[string]string{"status": "success"})
}

// GetUserByUsername - VULNERABLE: SQL injection via path parameter
// Taint flow: c.Param("username") -> string concat -> db.QueryRow
func (h *EchoHandler) GetUserByUsername(c echo.Context) error {
	// TAINT SOURCE: Path parameter from Echo
	username := c.Param("username")

	// TAINT PROPAGATION: Direct concatenation
	query := "SELECT * FROM users WHERE username = '" + username + "'"

	// TAINT SINK: SQL injection
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

// SearchUsers - VULNERABLE: SQL injection via form value
// Taint flow: c.FormValue("q") -> fmt.Sprintf -> db.Query
func (h *EchoHandler) SearchUsers(c echo.Context) error {
	// TAINT SOURCE: Form value from Echo
	searchTerm := c.FormValue("q")

	// TAINT PROPAGATION: Format string with LIKE
	query := fmt.Sprintf("SELECT * FROM users WHERE username LIKE '%%%s%%' OR email LIKE '%%%s%%'", searchTerm, searchTerm)

	// TAINT SINK: SQL injection
	_, err := h.db.Exec(query)
	if err != nil {
		return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}

	return c.JSON(http.StatusOK, map[string]string{"status": "search completed"})
}

// ===============================================
// TAINT SOURCE: c.Bind (JSON/Form binding)
// TAINT SINK: SQL injection via struct fields
// ===============================================

// CreateUser - VULNERABLE: SQL injection via JSON body
func (h *EchoHandler) CreateUser(c echo.Context) error {
	var input struct {
		Username string `json:"username"`
		Email    string `json:"email"`
		Role     string `json:"role"`
	}

	// TAINT SOURCE: JSON body binding via Echo
	if err := c.Bind(&input); err != nil {
		return c.JSON(http.StatusBadRequest, map[string]string{"error": err.Error()})
	}

	// TAINT PROPAGATION: Struct fields flow to query
	query := fmt.Sprintf(
		"INSERT INTO users (username, email, role) VALUES ('%s', '%s', '%s')",
		input.Username, input.Email, input.Role,
	)

	// TAINT SINK: SQL injection
	_, err := h.db.Exec(query)
	if err != nil {
		return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}

	return c.JSON(http.StatusCreated, map[string]string{"status": "created"})
}

// UpdateUser - VULNERABLE: SQL injection via PUT body
func (h *EchoHandler) UpdateUser(c echo.Context) error {
	// TAINT SOURCE: Path parameter
	userID := c.Param("id")

	var input struct {
		Email string `json:"email"`
		Role  string `json:"role"`
	}

	// TAINT SOURCE: JSON body
	if err := c.Bind(&input); err != nil {
		return c.JSON(http.StatusBadRequest, map[string]string{"error": err.Error()})
	}

	// TAINT PROPAGATION: Multiple sources combine
	query := fmt.Sprintf(
		"UPDATE users SET email = '%s', role = '%s' WHERE id = %s",
		input.Email, input.Role, userID,
	)

	// TAINT SINK: SQL injection from multiple sources
	_, err := h.db.Exec(query)
	if err != nil {
		return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}

	return c.JSON(http.StatusOK, map[string]string{"status": "updated"})
}

// ===============================================
// TAINT SINK: Command injection
// ===============================================

// RunDiagnostic - VULNERABLE: Command injection
// Taint flow: c.QueryParam("tool") -> exec.Command
func (h *EchoHandler) RunDiagnostic(c echo.Context) error {
	// TAINT SOURCE: Query parameters from Echo
	tool := c.QueryParam("tool")
	target := c.QueryParam("target")

	// TAINT SINK: Command injection - non-literal command
	cmd := exec.Command(tool, target)
	output, err := cmd.CombinedOutput()
	if err != nil {
		return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}

	return c.JSON(http.StatusOK, map[string]string{"output": string(output)})
}

// ExecShell - VULNERABLE: Shell command injection
func (h *EchoHandler) ExecShell(c echo.Context) error {
	// TAINT SOURCE: Form value
	shellCmd := c.FormValue("command")

	// TAINT SINK: Shell command injection
	cmd := exec.Command("bash", "-c", shellCmd)
	output, _ := cmd.CombinedOutput()

	return c.JSON(http.StatusOK, map[string]string{"output": string(output)})
}

// ===============================================
// TAINT SINK: Path traversal
// ===============================================

// ServeFile - VULNERABLE: Path traversal via query parameter
// Taint flow: c.QueryParam("file") -> filepath.Join -> os.Open
func (h *EchoHandler) ServeFile(c echo.Context) error {
	// TAINT SOURCE: Query parameter
	filename := c.QueryParam("file")

	// TAINT PROPAGATION: Path construction (still vulnerable to ../)
	filePath := filepath.Join("./static", filename)

	// TAINT SINK: Path traversal - arbitrary file read
	data, err := os.ReadFile(filePath)
	if err != nil {
		return c.JSON(http.StatusNotFound, map[string]string{"error": "file not found"})
	}

	return c.Blob(http.StatusOK, "application/octet-stream", data)
}

// DeleteFile - VULNERABLE: Path traversal on delete
func (h *EchoHandler) DeleteFile(c echo.Context) error {
	// TAINT SOURCE: Path parameter
	filename := c.Param("filename")

	// TAINT PROPAGATION: Path join
	filePath := filepath.Join("/tmp/uploads", filename)

	// TAINT SINK: Path traversal - arbitrary file deletion
	err := os.Remove(filePath)
	if err != nil {
		return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}

	return c.JSON(http.StatusOK, map[string]string{"status": "deleted"})
}

// ===============================================
// TAINT SOURCE: c.Request() - raw HTTP request
// ===============================================

// ProcessRequest - VULNERABLE: Raw request body to SQL
func (h *EchoHandler) ProcessRequest(c echo.Context) error {
	// TAINT SOURCE: Raw request via c.Request()
	req := c.Request()

	// Read header value
	customHeader := req.Header.Get("X-Custom-Data")

	// TAINT SINK: SQL injection from header
	query := fmt.Sprintf("INSERT INTO logs (data) VALUES ('%s')", customHeader)
	_, err := h.db.Exec(query)
	if err != nil {
		return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}

	return c.JSON(http.StatusOK, map[string]string{"status": "logged"})
}

// ===============================================
// COMPLEX FLOW: Cross-function taint propagation
// ===============================================

// ReportEndpoint - VULNERABLE: Cross-function taint flow
func (h *EchoHandler) ReportEndpoint(c echo.Context) error {
	// TAINT SOURCE: Query parameter
	reportType := c.QueryParam("type")
	dateRange := c.QueryParam("range")

	// TAINT PROPAGATION: Via function calls
	filters := h.buildFilters(reportType, dateRange)
	query := h.constructQuery("reports", filters)

	// TAINT SINK: SQL injection
	rows, err := h.db.Query(query)
	if err != nil {
		return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}
	defer rows.Close()

	return c.JSON(http.StatusOK, map[string]string{"status": "report generated"})
}

// buildFilters propagates taint through parameters
func (h *EchoHandler) buildFilters(reportType, dateRange string) string {
	// Taint propagates through return value
	return fmt.Sprintf("type = '%s' AND date_range = '%s'", reportType, dateRange)
}

// constructQuery builds the final vulnerable query
func (h *EchoHandler) constructQuery(table, filters string) string {
	// Taint flows into final query
	return fmt.Sprintf("SELECT * FROM %s WHERE %s", table, filters)
}

// ===============================================
// SECURE: Parameterized queries for comparison
// ===============================================

// GetUserSecure - SECURE: Uses parameterized query
func (h *EchoHandler) GetUserSecure(c echo.Context) error {
	userID := c.QueryParam("id")

	// SECURE: Parameterized query prevents injection
	query := "SELECT * FROM users WHERE id = ?"
	rows, err := h.db.Query(query, userID)
	if err != nil {
		return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}
	defer rows.Close()

	return c.JSON(http.StatusOK, map[string]string{"status": "success"})
}
