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

// GetUser handles user lookup by ID via Fiber
func (h *FiberHandler) GetUser(c *fiber.Ctx) error {
	userID := c.Query("id")

	query := fmt.Sprintf("SELECT * FROM users WHERE id = %s", userID)

	rows, err := h.db.Query(query)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}
	defer rows.Close()

	return c.JSON(fiber.Map{"status": "success"})
}

// GetUserByUsername looks up a user by username via Fiber
func (h *FiberHandler) GetUserByUsername(c *fiber.Ctx) error {
	username := c.Params("username")

	query := "SELECT * FROM users WHERE username = '" + username + "'"

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

// SearchUsers searches users by body content via Fiber
func (h *FiberHandler) SearchUsers(c *fiber.Ctx) error {
	searchTerm := string(c.Body())

	query := fmt.Sprintf("SELECT * FROM users WHERE username LIKE '%%%s%%'", searchTerm)

	rows, err := h.db.Query(query)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}
	defer rows.Close()

	return c.JSON(fiber.Map{"status": "search completed"})
}

// CreateUser creates a user from JSON body via Fiber
func (h *FiberHandler) CreateUser(c *fiber.Ctx) error {
	var input struct {
		Username string `json:"username"`
		Email    string `json:"email"`
		Role     string `json:"role"`
	}

	if err := c.BodyParser(&input); err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	query := fmt.Sprintf(
		"INSERT INTO users (username, email, role) VALUES ('%s', '%s', '%s')",
		input.Username, input.Email, input.Role,
	)

	_, err := h.db.Exec(query)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.Status(fiber.StatusCreated).JSON(fiber.Map{"status": "created"})
}

// UpdateUser updates a user's email and role via Fiber
func (h *FiberHandler) UpdateUser(c *fiber.Ctx) error {
	userID := c.Params("id")

	var input struct {
		Email string `json:"email"`
		Role  string `json:"role"`
	}

	if err := c.BodyParser(&input); err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	query := fmt.Sprintf(
		"UPDATE users SET email = '%s', role = '%s' WHERE id = %s",
		input.Email, input.Role, userID,
	)

	_, err := h.db.Exec(query)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.JSON(fiber.Map{"status": "updated"})
}

// DeleteUser deletes a user by ID via Fiber
func (h *FiberHandler) DeleteUser(c *fiber.Ctx) error {
	userID := c.Params("id")

	query := fmt.Sprintf("DELETE FROM users WHERE id = %s", userID)

	_, err := h.db.Exec(query)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.SendStatus(fiber.StatusNoContent)
}

// RunCommand executes a command from query params via Fiber
func (h *FiberHandler) RunCommand(c *fiber.Ctx) error {
	cmd := c.Query("cmd")
	args := c.Query("args")

	command := exec.Command(cmd, args)
	output, err := command.CombinedOutput()
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.JSON(fiber.Map{"output": string(output)})
}

// ExecShell executes a shell command from request body
func (h *FiberHandler) ExecShell(c *fiber.Ctx) error {
	shellCmd := string(c.Body())

	cmd := exec.Command("sh", "-c", shellCmd)
	output, _ := cmd.CombinedOutput()

	return c.Send(output)
}

// ServeFile serves a file from the public directory via Fiber
func (h *FiberHandler) ServeFile(c *fiber.Ctx) error {
	filename := c.Params("*")

	filePath := filepath.Join("./public", filename)

	data, err := os.ReadFile(filePath)
	if err != nil {
		return c.Status(fiber.StatusNotFound).JSON(fiber.Map{
			"error": "file not found",
		})
	}

	return c.Send(data)
}

// DownloadFile serves a file for download via Fiber
func (h *FiberHandler) DownloadFile(c *fiber.Ctx) error {
	filename := c.Query("file")

	filePath := filepath.Join("/downloads", filename)

	return c.SendFile(filePath)
}

// ProcessHeaders logs request headers to the database
func (h *FiberHandler) ProcessHeaders(c *fiber.Ctx) error {
	userAgent := c.Get("User-Agent")
	customHeader := c.Get("X-Custom-Data")

	query := fmt.Sprintf(
		"INSERT INTO logs (user_agent, custom_data) VALUES ('%s', '%s')",
		userAgent, customHeader,
	)

	_, err := h.db.Exec(query)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.JSON(fiber.Map{"status": "logged"})
}

// ProcessCookies queries session data from a cookie
func (h *FiberHandler) ProcessCookies(c *fiber.Ctx) error {
	sessionData := c.Cookies("session_data")

	query := fmt.Sprintf("SELECT * FROM sessions WHERE data = '%s'", sessionData)

	rows, err := h.db.Query(query)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": err.Error(),
		})
	}
	defer rows.Close()

	return c.JSON(fiber.Map{"status": "authenticated"})
}

// BatchOperation processes a batch of inserts
func (h *FiberHandler) BatchOperation(c *fiber.Ctx) error {
	var input struct {
		TableName  string   `json:"table"`
		ColumnName string   `json:"column"`
		Values     []string `json:"values"`
	}

	if err := c.BodyParser(&input); err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	for _, value := range input.Values {
		query := fmt.Sprintf(
			"INSERT INTO %s (%s) VALUES ('%s')",
			input.TableName, input.ColumnName, value,
		)

		if _, err := h.db.Exec(query); err != nil {
			return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
				"error": err.Error(),
			})
		}
	}

	return c.JSON(fiber.Map{"status": "batch completed"})
}

// GetUserSecure looks up a user with parameterized query
func (h *FiberHandler) GetUserSecure(c *fiber.Ctx) error {
	userID := c.Query("id")

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

// CreateUserSecure creates a user with prepared statement
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
