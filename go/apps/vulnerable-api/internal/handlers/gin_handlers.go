package handlers

import (
	"database/sql"
	"fmt"
	"html/template"
	"io"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"

	"github.com/gin-gonic/gin"
	"github.com/theauditor/vulnerable-api/internal/models"
	"github.com/theauditor/vulnerable-api/internal/repository"
)

// GinHandler handles HTTP requests using Gin framework
type GinHandler struct {
	userRepo *repository.UserRepository
	db       *sql.DB
}

// NewGinHandler creates a new Gin handler
func NewGinHandler(userRepo *repository.UserRepository, db *sql.DB) *GinHandler {
	return &GinHandler{
		userRepo: userRepo,
		db:       db,
	}
}

// GetUser handles user lookup by ID via Gin
func (h *GinHandler) GetUser(c *gin.Context) {
	userID := c.Query("id")

	query := fmt.Sprintf("SELECT * FROM users WHERE id = %s", userID)

	rows, err := h.db.Query(query)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}
	defer rows.Close()

	c.JSON(http.StatusOK, gin.H{"status": "success"})
}

// GetUserByUsername looks up a user by username via Gin
func (h *GinHandler) GetUserByUsername(c *gin.Context) {
	username := c.Param("username")

	query := "SELECT * FROM users WHERE username = '" + username + "'"

	row := h.db.QueryRow(query)
	var user models.User
	_ = row.Scan(&user.ID, &user.Username, &user.Email)

	c.JSON(http.StatusOK, user)
}

// SearchUsers searches users by username via Gin
func (h *GinHandler) SearchUsers(c *gin.Context) {
	searchTerm := c.PostForm("q")

	query := fmt.Sprintf("SELECT * FROM users WHERE username LIKE '%%%s%%'", searchTerm)

	_, err := h.db.Exec(query)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"status": "search completed"})
}

// CreateUser creates a user from JSON body via Gin
func (h *GinHandler) CreateUser(c *gin.Context) {
	var input struct {
		Username string `json:"username"`
		Email    string `json:"email"`
		Role     string `json:"role"`
	}

	if err := c.ShouldBind(&input); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	query := fmt.Sprintf(
		"INSERT INTO users (username, email, role) VALUES ('%s', '%s', '%s')",
		input.Username, input.Email, input.Role,
	)

	_, err := h.db.Exec(query)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusCreated, gin.H{"status": "created"})
}

// RunCommand executes a command from query params via Gin
func (h *GinHandler) RunCommand(c *gin.Context) {
	command := c.Query("cmd")
	args := c.Query("args")

	cmd := exec.Command(command, args)
	output, err := cmd.Output()
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"output": string(output)})
}

// PingHost pings a host via shell command
func (h *GinHandler) PingHost(c *gin.Context) {
	host := c.Param("host")

	shellCmd := fmt.Sprintf("ping -c 1 %s", host)

	cmd := exec.Command("sh", "-c", shellCmd)
	output, _ := cmd.CombinedOutput()

	c.JSON(http.StatusOK, gin.H{"output": string(output)})
}

// RenderProfile renders a profile page
func (h *GinHandler) RenderProfile(c *gin.Context) {
	name := c.Query("name")
	bio := c.Query("bio")

	unsafeName := template.HTML(name)
	unsafeBio := template.HTML(bio)

	c.HTML(http.StatusOK, "profile.html", gin.H{
		"name": unsafeName,
		"bio":  unsafeBio,
	})
}

// RenderJS renders a script template
func (h *GinHandler) RenderJS(c *gin.Context) {
	callback := c.Query("callback")

	unsafeJS := template.JS(callback)

	c.HTML(http.StatusOK, "script.html", gin.H{
		"callback": unsafeJS,
	})
}

// DownloadFile serves a file from uploads directory
func (h *GinHandler) DownloadFile(c *gin.Context) {
	filename := c.Param("filename")

	filePath := filepath.Join("/uploads", filename)

	file, err := os.Open(filePath)
	if err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "file not found"})
		return
	}
	defer file.Close()

	c.File(filePath)
}

// ReadConfig reads a config file by path
func (h *GinHandler) ReadConfig(c *gin.Context) {
	configPath := c.Query("path")

	content, err := os.ReadFile(configPath)
	if err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"content": string(content)})
}

// SaveFile saves request body to a file
func (h *GinHandler) SaveFile(c *gin.Context) {
	savePath := c.PostForm("path")

	file, err := os.Create(savePath)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}
	defer file.Close()

	// Read request body and write to file
	body, _ := io.ReadAll(c.Request.Body)
	file.Write(body)

	c.JSON(http.StatusOK, gin.H{"status": "saved"})
}

// ComplexFlow processes input through multiple steps
func (h *GinHandler) ComplexFlow(c *gin.Context) {
	userInput := c.Query("data")

	processed := processInput(userInput)

	query := buildQuery(processed)

	_, err := h.db.Exec(query)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"status": "processed"})
}

// processInput is a helper that propagates taint
func processInput(input string) string {
	// Taint propagates through function return
	return "value_" + input
}

// buildQuery is a helper that creates vulnerable SQL
func buildQuery(value string) string {
	// Taint flows into SQL construction
	return fmt.Sprintf("INSERT INTO logs (value) VALUES ('%s')", value)
}

// GetUserSecure looks up a user with parameterized query
func (h *GinHandler) GetUserSecure(c *gin.Context) {
	userID := c.Query("id")

	query := "SELECT * FROM users WHERE id = ?"
	rows, err := h.db.Query(query, userID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}
	defer rows.Close()

	c.JSON(http.StatusOK, gin.H{"status": "success"})
}

// RunCommandSecure executes ping with user-provided host
func (h *GinHandler) RunCommandSecure(c *gin.Context) {
	host := c.Query("host")

	// Validate host is IP or hostname only (no special chars)
	// This is still risky but demonstrates validation
	cmd := exec.Command("ping", "-c", "1", host)
	output, err := cmd.Output()
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"output": string(output)})
}
