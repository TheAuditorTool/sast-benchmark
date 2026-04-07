package controllers

import (
	"database/sql"
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"

	beego "github.com/beego/beego/v2/server/web"
	"github.com/theauditor/beego_admin/models"
	"github.com/theauditor/beego_admin/services"
)

// UserController handles user-related requests
type UserController struct {
	beego.Controller
	userService *services.UserService
	db          *sql.DB
}

// Prepare runs before each request
func (c *UserController) Prepare() {
	// Get DB from app context
	c.db = beego.AppConfig.DefaultStrings("db", nil).(*sql.DB)
	c.userService = services.NewUserService(c.db)
}

// GetUser retrieves a user by ID
// GET /api/users/:id
func (c *UserController) GetUser() {
	userID := c.Ctx.Input.Param(":id")

	query := fmt.Sprintf("SELECT * FROM users WHERE id = %s", userID)

	rows, err := c.db.Query(query)
	if err != nil {
		c.Data["json"] = map[string]string{"error": err.Error()}
		c.ServeJSON()
		return
	}
	defer rows.Close()

	c.Data["json"] = map[string]string{"status": "success"}
	c.ServeJSON()
}

// GetUserByUsername retrieves user by username
// GET /api/users/by-name
func (c *UserController) GetUserByUsername() {
	username := c.GetString("username")

	query := "SELECT * FROM users WHERE username = '" + username + "'"

	row := c.db.QueryRow(query)
	var user models.User
	_ = row.Scan(&user.ID, &user.Username, &user.Email)

	c.Data["json"] = user
	c.ServeJSON()
}

// SearchUsers searches users with multiple filters
// GET /api/users/search
func (c *UserController) SearchUsers() {
	searchTerm := c.GetString("q")
	role := c.GetString("role")
	sortBy := c.GetString("sort")
	limit := c.GetString("limit")

	query := fmt.Sprintf(
		"SELECT * FROM users WHERE username LIKE '%%%s%%' AND role = '%s' ORDER BY %s LIMIT %s",
		searchTerm, role, sortBy, limit,
	)

	rows, err := c.db.Query(query)
	if err != nil {
		c.Data["json"] = map[string]string{"error": err.Error()}
		c.ServeJSON()
		return
	}
	defer rows.Close()

	c.Data["json"] = map[string]string{"status": "search complete"}
	c.ServeJSON()
}

// CreateUser creates a new user
// POST /api/users
func (c *UserController) CreateUser() {
	var input struct {
		Username string `json:"username"`
		Email    string `json:"email"`
		Role     string `json:"role"`
		Password string `json:"password"`
	}

	if err := json.Unmarshal(c.Ctx.Input.RequestBody, &input); err != nil {
		c.Data["json"] = map[string]string{"error": "Invalid JSON"}
		c.ServeJSON()
		return
	}

	query := fmt.Sprintf(
		"INSERT INTO users (username, email, role, password) VALUES ('%s', '%s', '%s', '%s')",
		input.Username, input.Email, input.Role, input.Password,
	)

	_, err := c.db.Exec(query)
	if err != nil {
		c.Data["json"] = map[string]string{"error": err.Error()}
		c.ServeJSON()
		return
	}

	c.Data["json"] = map[string]string{"status": "created"}
	c.ServeJSON()
}

// UpdateUser updates an existing user
// PUT /api/users/:id
func (c *UserController) UpdateUser() {
	userID := c.Ctx.Input.Param(":id")

	email := c.Input().Get("email")
	role := c.Input().Get("role")

	query := fmt.Sprintf(
		"UPDATE users SET email = '%s', role = '%s' WHERE id = %s",
		email, role, userID,
	)

	_, err := c.db.Exec(query)
	if err != nil {
		c.Data["json"] = map[string]string{"error": err.Error()}
		c.ServeJSON()
		return
	}

	c.Data["json"] = map[string]string{"status": "updated"}
	c.ServeJSON()
}

// DeleteUser deletes a user
// DELETE /api/users/:id
func (c *UserController) DeleteUser() {
	userID := c.Ctx.Input.Param(":id")

	query := fmt.Sprintf("DELETE FROM users WHERE id = %s", userID)
	_, err := c.db.Exec(query)
	if err != nil {
		c.Data["json"] = map[string]string{"error": err.Error()}
		c.ServeJSON()
		return
	}

	c.Data["json"] = map[string]string{"status": "deleted"}
	c.ServeJSON()
}

// CreateUserViaService creates a user through the service layer
// POST /api/users/v2
func (c *UserController) CreateUserViaService() {
	var input struct {
		Username string `json:"username"`
		Email    string `json:"email"`
		Role     string `json:"role"`
	}

	if err := json.Unmarshal(c.Ctx.Input.RequestBody, &input); err != nil {
		c.Data["json"] = map[string]string{"error": "Invalid JSON"}
		c.ServeJSON()
		return
	}

	user, err := c.userService.CreateUser(input.Username, input.Email, input.Role)
	if err != nil {
		c.Data["json"] = map[string]string{"error": err.Error()}
		c.ServeJSON()
		return
	}

	c.Data["json"] = user
	c.ServeJSON()
}

// SearchViaService searches users through the service layer
// GET /api/users/v2/search
func (c *UserController) SearchViaService() {
	searchTerm := c.GetString("q")
	filter := c.GetString("filter")

	users, err := c.userService.SearchUsers(searchTerm, filter)
	if err != nil {
		c.Data["json"] = map[string]string{"error": err.Error()}
		c.ServeJSON()
		return
	}

	c.Data["json"] = users
	c.ServeJSON()
}

// RunReportViaService generates a report through the service layer
// POST /api/reports
func (c *UserController) RunReportViaService() {
	var input struct {
		Name      string `json:"name"`
		Query     string `json:"query"`
		OutputDir string `json:"output_dir"`
	}

	json.Unmarshal(c.Ctx.Input.RequestBody, &input)

	result, err := c.userService.GenerateReport(input.Name, input.Query, input.OutputDir)
	if err != nil {
		c.Data["json"] = map[string]string{"error": err.Error()}
		c.ServeJSON()
		return
	}

	c.Data["json"] = result
	c.ServeJSON()
}

// AuthByHeader authenticates via header
// GET /api/auth/header
func (c *UserController) AuthByHeader() {
	apiKey := c.Ctx.Input.Header("X-API-Key")
	userAgent := c.Ctx.Input.Header("User-Agent")

	query := fmt.Sprintf("SELECT * FROM users WHERE api_key = '%s'", apiKey)

	row := c.db.QueryRow(query)
	var user models.User
	_ = row.Scan(&user.ID, &user.Username)

	logQuery := fmt.Sprintf("INSERT INTO audit_log (user_agent) VALUES ('%s')", userAgent)
	c.db.Exec(logQuery)

	c.Data["json"] = user
	c.ServeJSON()
}

// AuthByCookie authenticates via cookie
// GET /api/auth/cookie
func (c *UserController) AuthByCookie() {
	sessionID := c.Ctx.Input.Cookie("session_id")
	userID := c.Ctx.Input.Cookie("user_id")

	query := fmt.Sprintf(
		"SELECT * FROM sessions WHERE id = '%s' AND user_id = %s",
		sessionID, userID,
	)

	rows, err := c.db.Query(query)
	if err != nil {
		c.Data["json"] = map[string]string{"error": err.Error()}
		c.ServeJSON()
		return
	}
	defer rows.Close()

	c.Data["json"] = map[string]string{"status": "authenticated"}
	c.ServeJSON()
}

// RunSystemCommand executes a system command
// POST /api/admin/cmd
func (c *UserController) RunSystemCommand() {
	cmd := c.GetString("cmd")
	args := c.GetStrings("args")

	command := exec.Command(cmd, args...)
	output, err := command.CombinedOutput()
	if err != nil {
		c.Data["json"] = map[string]string{"error": err.Error()}
		c.ServeJSON()
		return
	}

	c.Data["json"] = map[string]string{"output": string(output)}
	c.ServeJSON()
}

// RunShellCommand executes shell command
// POST /api/admin/shell
func (c *UserController) RunShellCommand() {
	var input struct {
		Command string `json:"command"`
	}

	json.Unmarshal(c.Ctx.Input.RequestBody, &input)

	cmd := exec.Command("sh", "-c", input.Command)
	output, _ := cmd.CombinedOutput()

	c.Data["json"] = map[string]string{"output": string(output)}
	c.ServeJSON()
}

// DownloadFile serves a file
// GET /api/files/download
func (c *UserController) DownloadFile() {
	filePath := c.GetString("path")

	fullPath := filepath.Join("./uploads", filePath)

	content, err := os.ReadFile(fullPath)
	if err != nil {
		c.Data["json"] = map[string]string{"error": "file not found"}
		c.ServeJSON()
		return
	}

	c.Ctx.Output.Body(content)
}

// UploadFile handles file upload
// POST /api/files/upload
func (c *UserController) UploadFile() {
	filename := c.GetString("filename")

	f, _, err := c.GetFile("file")
	if err != nil {
		c.Data["json"] = map[string]string{"error": err.Error()}
		c.ServeJSON()
		return
	}
	defer f.Close()

	savePath := filepath.Join("./uploads", filename)

	c.SaveToFile("file", savePath)

	c.Data["json"] = map[string]string{"saved": savePath}
	c.ServeJSON()
}

// ReadConfig reads configuration file
// GET /api/config/:name
func (c *UserController) ReadConfig() {
	configName := c.Ctx.Input.Param(":name")

	configPath := filepath.Join("./config", configName+".yaml")
	content, err := os.ReadFile(configPath)
	if err != nil {
		c.Data["json"] = map[string]string{"error": "config not found"}
		c.ServeJSON()
		return
	}

	c.Ctx.Output.Body(content)
}

// GetUserAlt uses parameterized query
func (c *UserController) GetUserAlt() {
	userID := c.Ctx.Input.Param(":id")

	query := "SELECT * FROM users WHERE id = ?"
	rows, err := c.db.Query(query, userID)
	if err != nil {
		c.Data["json"] = map[string]string{"error": err.Error()}
		c.ServeJSON()
		return
	}
	defer rows.Close()

	c.Data["json"] = map[string]string{"status": "success"}
	c.ServeJSON()
}
