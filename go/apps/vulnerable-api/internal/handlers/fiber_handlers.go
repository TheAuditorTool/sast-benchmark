package handlers

import (
	"database/sql"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"

	"github.com/gofiber/fiber/v2"
	"github.com/theauditor/vulnerable-api/internal/repository"
)

// FiberHandler handles HTTP requests using Fiber framework
type FiberHandler struct {
	userRepo *repository.UserRepository
	db       *sql.DB
}

// NewFiberHandler creates a new Fiber handler
func NewFiberHandler(userRepo *repository.UserRepository, db *sql.DB) *FiberHandler {
	return &FiberHandler{
		userRepo: userRepo,
		db:       db,
	}
}

// ===============================================
// TAINT SOURCE: c.Params, c.Query, c.Body
// TAINT SINK: SQL injection
// ===============================================

// GetUser - VULNERABLE: SQL injection via query parameter
// Taint flow: c.Query("id") -> fmt.Sprintf -> db.Query
func (h *FiberHandler) GetUser(c *fiber.Ctx) error {
	// TAINT SOURCE: Query parameter from Fiber context
	userID := c.Query("id")

	// TAINT PROPAGATION: User input flows into SQL query
	query := fmt.Sprintf("SELECT * FROM users WHERE id = %s", userID)

	// TAINT SINK: SQL injection
	rows, err := h.db.Query(query)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}
	defer rows.Close()

	return c.JSON(fiber.Map{"status": "success"})
}

// GetUserByUsername - VULNERABLE: SQL injection via path parameter
// Taint flow: c.Params("username") -> string concat -> db.QueryRow
func (h *FiberHandler) GetUserByUsername(c *fiber.Ctx) error {
	// TAINT SOURCE: Path parameter from Fiber
	username := c.Params("username")

	// TAINT PROPAGATION: Direct concatenation
	query := "SELECT * FROM users WHERE username = '" + username + "'"

	// TAINT SINK: SQL injection
	row := h.db.QueryRow(query)
	var id int
	var name, email string
	_ = row.Scan(&id, &name, &email)

	return c.JSON(fiber.Map{
		"id":       id,
		"username": name,
		"email":    email,
	})
}

// SearchUsers - VULNERABLE: SQL injection via body
// Taint flow: c.Body() -> fmt.Sprintf -> db.Query
func (h *FiberHandler) SearchUsers(c *fiber.Ctx) error {
	// TAINT SOURCE: Request body from Fiber
	searchTerm := string(c.Body())

	// TAINT PROPAGATION: Body content in query
	query := fmt.Sprintf("SELECT * FROM users WHERE username LIKE '%%%s%%'", searchTerm)

	// TAINT SINK: SQL injection
	rows, err := h.db.Query(query)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}
	defer rows.Close()

	return c.JSON(fiber.Map{"status": "search completed"})
}

// ===============================================
// TAINT SOURCE: c.BodyParser (JSON binding)
// TAINT SINK: SQL injection via struct fields
// ===============================================

// CreateUser - VULNERABLE: SQL injection via JSON body
func (h *FiberHandler) CreateUser(c *fiber.Ctx) error {
	var input struct {
		Username string `json:"username"`
		Email    string `json:"email"`
		Role     string `json:"role"`
	}

	// TAINT SOURCE: JSON body parsing via Fiber
	if err := c.BodyParser(&input); err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	// TAINT PROPAGATION: Struct fields flow to query
	query := fmt.Sprintf(
		"INSERT INTO users (username, email, role) VALUES ('%s', '%s', '%s')",
		input.Username, input.Email, input.Role,
	)

	// TAINT SINK: SQL injection
	_, err := h.db.Exec(query)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.Status(fiber.StatusCreated).JSON(fiber.Map{"status": "created"})
}

// UpdateUser - VULNERABLE: SQL injection via params + body
func (h *FiberHandler) UpdateUser(c *fiber.Ctx) error {
	// TAINT SOURCE: Path parameter
	userID := c.Params("id")

	var input struct {
		Email string `json:"email"`
		Role  string `json:"role"`
	}

	// TAINT SOURCE: JSON body
	if err := c.BodyParser(&input); err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	// TAINT PROPAGATION: Multiple sources combine
	query := fmt.Sprintf(
		"UPDATE users SET email = '%s', role = '%s' WHERE id = %s",
		input.Email, input.Role, userID,
	)

	// TAINT SINK: SQL injection from multiple sources
	_, err := h.db.Exec(query)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.JSON(fiber.Map{"status": "updated"})
}

// DeleteUser - VULNERABLE: SQL injection via DELETE
func (h *FiberHandler) DeleteUser(c *fiber.Ctx) error {
	// TAINT SOURCE: Path parameter
	userID := c.Params("id")

	// TAINT PROPAGATION: Direct to query
	query := fmt.Sprintf("DELETE FROM users WHERE id = %s", userID)

	// TAINT SINK: SQL injection
	_, err := h.db.Exec(query)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.SendStatus(fiber.StatusNoContent)
}

// ===============================================
// TAINT SINK: Command injection
// ===============================================

// RunCommand - VULNERABLE: Command injection via query
// Taint flow: c.Query -> exec.Command
func (h *FiberHandler) RunCommand(c *fiber.Ctx) error {
	// TAINT SOURCE: Query parameters
	cmd := c.Query("cmd")
	args := c.Query("args")

	// TAINT SINK: Command injection - non-literal command
	command := exec.Command(cmd, args)
	output, err := command.CombinedOutput()
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.JSON(fiber.Map{"output": string(output)})
}

// ExecShell - VULNERABLE: Shell command injection
func (h *FiberHandler) ExecShell(c *fiber.Ctx) error {
	// TAINT SOURCE: Request body
	shellCmd := string(c.Body())

	// TAINT SINK: Shell command injection
	cmd := exec.Command("sh", "-c", shellCmd)
	output, _ := cmd.CombinedOutput()

	return c.Send(output)
}

// ===============================================
// TAINT SINK: Path traversal
// ===============================================

// ServeFile - VULNERABLE: Path traversal via params
// Taint flow: c.Params -> filepath.Join -> os.Open
func (h *FiberHandler) ServeFile(c *fiber.Ctx) error {
	// TAINT SOURCE: Path parameter (wildcard)
	filename := c.Params("*")

	// TAINT PROPAGATION: Path construction
	filePath := filepath.Join("./public", filename)

	// TAINT SINK: Path traversal
	data, err := os.ReadFile(filePath)
	if err != nil {
		return c.Status(fiber.StatusNotFound).JSON(fiber.Map{
			"error": "file not found",
		})
	}

	return c.Send(data)
}

// DownloadFile - VULNERABLE: Path traversal via query
func (h *FiberHandler) DownloadFile(c *fiber.Ctx) error {
	// TAINT SOURCE: Query parameter
	filename := c.Query("file")

	// TAINT PROPAGATION: Direct path usage
	filePath := filepath.Join("/downloads", filename)

	// TAINT SINK: Path traversal - file read
	return c.SendFile(filePath)
}

// ===============================================
// TAINT SOURCE: Headers and Cookies
// ===============================================

// ProcessHeaders - VULNERABLE: Header to SQL
func (h *FiberHandler) ProcessHeaders(c *fiber.Ctx) error {
	// TAINT SOURCE: HTTP headers via Fiber
	userAgent := c.Get("User-Agent")
	customHeader := c.Get("X-Custom-Data")

	// TAINT PROPAGATION: Headers to query
	query := fmt.Sprintf(
		"INSERT INTO logs (user_agent, custom_data) VALUES ('%s', '%s')",
		userAgent, customHeader,
	)

	// TAINT SINK: SQL injection via headers
	_, err := h.db.Exec(query)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.JSON(fiber.Map{"status": "logged"})
}

// ProcessCookies - VULNERABLE: Cookie to SQL
func (h *FiberHandler) ProcessCookies(c *fiber.Ctx) error {
	// TAINT SOURCE: Cookie value via Fiber
	sessionData := c.Cookies("session_data")

	// TAINT PROPAGATION: Cookie value to query
	query := fmt.Sprintf("SELECT * FROM sessions WHERE data = '%s'", sessionData)

	// TAINT SINK: SQL injection via cookie
	rows, err := h.db.Query(query)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}
	defer rows.Close()

	return c.JSON(fiber.Map{"status": "authenticated"})
}

// ===============================================
// COMPLEX FLOW: Multi-step propagation
// ===============================================

// BatchOperation - VULNERABLE: Batch processing with taint
func (h *FiberHandler) BatchOperation(c *fiber.Ctx) error {
	var input struct {
		TableName  string   `json:"table"`
		ColumnName string   `json:"column"`
		Values     []string `json:"values"`
	}

	// TAINT SOURCE: JSON body
	if err := c.BodyParser(&input); err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	// TAINT PROPAGATION: Batch insert construction
	for _, value := range input.Values {
		// Each iteration propagates taint
		query := fmt.Sprintf(
			"INSERT INTO %s (%s) VALUES ('%s')",
			input.TableName, input.ColumnName, value,
		)

		// TAINT SINK: SQL injection in loop
		if _, err := h.db.Exec(query); err != nil {
			return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
				"error": err.Error(),
			})
		}
	}

	return c.JSON(fiber.Map{"status": "batch completed"})
}

// ===============================================
// SECURE: Parameterized queries for comparison
// ===============================================

// GetUserSecure - SECURE: Uses parameterized query
func (h *FiberHandler) GetUserSecure(c *fiber.Ctx) error {
	userID := c.Query("id")

	// SECURE: Parameterized query
	query := "SELECT * FROM users WHERE id = ?"
	rows, err := h.db.Query(query, userID)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}
	defer rows.Close()

	return c.JSON(fiber.Map{"status": "success"})
}

// CreateUserSecure - SECURE: Uses Prepare statement
func (h *FiberHandler) CreateUserSecure(c *fiber.Ctx) error {
	var input struct {
		Username string `json:"username"`
		Email    string `json:"email"`
	}

	if err := c.BodyParser(&input); err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	// SECURE: Prepared statement
	stmt, err := h.db.Prepare("INSERT INTO users (username, email) VALUES (?, ?)")
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}
	defer stmt.Close()

	_, err = stmt.Exec(input.Username, input.Email)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.Status(fiber.StatusCreated).JSON(fiber.Map{"status": "created"})
}
