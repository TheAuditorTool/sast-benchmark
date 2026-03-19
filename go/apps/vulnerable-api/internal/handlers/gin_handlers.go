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

// ===============================================
// TAINT SOURCE: c.Query, c.Param, c.PostForm
// TAINT SINK: SQL injection
// ===============================================

// GetUser - VULNERABLE: SQL injection via query parameter
// Taint flow: c.Query("id") -> fmt.Sprintf -> db.Query
func (h *GinHandler) GetUser(c *gin.Context) {
	// TAINT SOURCE: Query parameter from user
	userID := c.Query("id")

	// TAINT PROPAGATION: User input flows into SQL query
	query := fmt.Sprintf("SELECT * FROM users WHERE id = %s", userID)

	// TAINT SINK: SQL injection
	rows, err := h.db.Query(query)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}
	defer rows.Close()

	c.JSON(http.StatusOK, gin.H{"status": "success"})
}

// GetUserByUsername - VULNERABLE: SQL injection via path parameter
// Taint flow: c.Param("username") -> string concat -> db.QueryRow
func (h *GinHandler) GetUserByUsername(c *gin.Context) {
	// TAINT SOURCE: Path parameter
	username := c.Param("username")

	// TAINT PROPAGATION: Direct concatenation
	query := "SELECT * FROM users WHERE username = '" + username + "'"

	// TAINT SINK: SQL injection
	row := h.db.QueryRow(query)
	var user models.User
	_ = row.Scan(&user.ID, &user.Username, &user.Email)

	c.JSON(http.StatusOK, user)
}

// SearchUsers - VULNERABLE: SQL injection via search form
// Taint flow: c.PostForm("q") -> fmt.Sprintf -> db.Query
func (h *GinHandler) SearchUsers(c *gin.Context) {
	// TAINT SOURCE: Form data
	searchTerm := c.PostForm("q")

	// TAINT PROPAGATION: Format string
	query := fmt.Sprintf("SELECT * FROM users WHERE username LIKE '%%%s%%'", searchTerm)

	// TAINT SINK: SQL injection
	_, err := h.db.Exec(query)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"status": "search completed"})
}

// ===============================================
// TAINT SOURCE: c.ShouldBind, c.BindJSON
// TAINT SINK: SQL injection via struct fields
// ===============================================

// CreateUser - VULNERABLE: SQL injection via JSON body
func (h *GinHandler) CreateUser(c *gin.Context) {
	var input struct {
		Username string `json:"username"`
		Email    string `json:"email"`
		Role     string `json:"role"`
	}

	// TAINT SOURCE: JSON body binding
	if err := c.ShouldBind(&input); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	// TAINT PROPAGATION: Struct fields flow to query
	query := fmt.Sprintf(
		"INSERT INTO users (username, email, role) VALUES ('%s', '%s', '%s')",
		input.Username, input.Email, input.Role,
	)

	// TAINT SINK: SQL injection
	_, err := h.db.Exec(query)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusCreated, gin.H{"status": "created"})
}

// ===============================================
// TAINT SINK: Command injection
// ===============================================

// RunCommand - VULNERABLE: Command injection via query parameter
// Taint flow: c.Query("cmd") -> exec.Command
func (h *GinHandler) RunCommand(c *gin.Context) {
	// TAINT SOURCE: Query parameter
	command := c.Query("cmd")
	args := c.Query("args")

	// TAINT SINK: Command injection - non-literal command
	cmd := exec.Command(command, args)
	output, err := cmd.Output()
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"output": string(output)})
}

// PingHost - VULNERABLE: Command injection via shell
// Taint flow: c.Param("host") -> exec.Command("sh", "-c", ...)
func (h *GinHandler) PingHost(c *gin.Context) {
	// TAINT SOURCE: Path parameter
	host := c.Param("host")

	// TAINT PROPAGATION: Shell command construction
	shellCmd := fmt.Sprintf("ping -c 1 %s", host)

	// TAINT SINK: Command injection via shell
	cmd := exec.Command("sh", "-c", shellCmd)
	output, _ := cmd.CombinedOutput()

	c.JSON(http.StatusOK, gin.H{"output": string(output)})
}

// ===============================================
// TAINT SINK: Template injection (XSS)
// ===============================================

// RenderProfile - VULNERABLE: XSS via unsafe template
// Taint flow: c.Query("name") -> template.HTML -> response
func (h *GinHandler) RenderProfile(c *gin.Context) {
	// TAINT SOURCE: Query parameter
	name := c.Query("name")
	bio := c.Query("bio")

	// TAINT SINK: XSS - bypassing Go's template auto-escaping
	// template.HTML marks content as safe, disabling escaping
	unsafeName := template.HTML(name)
	unsafeBio := template.HTML(bio)

	c.HTML(http.StatusOK, "profile.html", gin.H{
		"name": unsafeName,
		"bio":  unsafeBio,
	})
}

// RenderJS - VULNERABLE: JavaScript injection
func (h *GinHandler) RenderJS(c *gin.Context) {
	// TAINT SOURCE: Query parameter
	callback := c.Query("callback")

	// TAINT SINK: JavaScript injection
	unsafeJS := template.JS(callback)

	c.HTML(http.StatusOK, "script.html", gin.H{
		"callback": unsafeJS,
	})
}

// ===============================================
// TAINT SINK: Path traversal
// ===============================================

// DownloadFile - VULNERABLE: Path traversal via filename
// Taint flow: c.Param("filename") -> filepath.Join -> os.Open
func (h *GinHandler) DownloadFile(c *gin.Context) {
	// TAINT SOURCE: Path parameter
	filename := c.Param("filename")

	// TAINT PROPAGATION: Path construction
	// Even with filepath.Join, path traversal is possible with ../
	filePath := filepath.Join("/uploads", filename)

	// TAINT SINK: Path traversal
	file, err := os.Open(filePath)
	if err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "file not found"})
		return
	}
	defer file.Close()

	c.File(filePath)
}

// ReadConfig - VULNERABLE: Path traversal via query parameter
// Taint flow: c.Query("path") -> os.ReadFile
func (h *GinHandler) ReadConfig(c *gin.Context) {
	// TAINT SOURCE: Query parameter
	configPath := c.Query("path")

	// TAINT SINK: Path traversal - direct file read
	content, err := os.ReadFile(configPath)
	if err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"content": string(content)})
}

// SaveFile - VULNERABLE: Path traversal on write
// Taint flow: c.PostForm("path") -> os.Create
func (h *GinHandler) SaveFile(c *gin.Context) {
	// TAINT SOURCE: Form data
	savePath := c.PostForm("path")

	// TAINT SINK: Path traversal - arbitrary file write
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

// ===============================================
// COMPLEX TAINT FLOW: Multi-hop propagation
// ===============================================

// ComplexFlow - VULNERABLE: Multi-step taint propagation
// Demonstrates: source -> variable -> function -> sink
func (h *GinHandler) ComplexFlow(c *gin.Context) {
	// TAINT SOURCE: Query parameter
	userInput := c.Query("data")

	// TAINT PROPAGATION: Step 1 - Assignment
	processed := processInput(userInput)

	// TAINT PROPAGATION: Step 2 - String formatting
	query := buildQuery(processed)

	// TAINT SINK: SQL injection
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

// ===============================================
// SECURE: Examples with proper sanitization
// ===============================================

// GetUserSecure - SECURE: Uses parameterized query
func (h *GinHandler) GetUserSecure(c *gin.Context) {
	userID := c.Query("id")

	// SECURE: Parameterized query
	query := "SELECT * FROM users WHERE id = ?"
	rows, err := h.db.Query(query, userID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}
	defer rows.Close()

	c.JSON(http.StatusOK, gin.H{"status": "success"})
}

// RunCommandSecure - SECURE: Uses hardcoded command
func (h *GinHandler) RunCommandSecure(c *gin.Context) {
	// SECURE: Hardcoded command, only arg is user input
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
