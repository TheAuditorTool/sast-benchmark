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

// ===============================================
// BEEGO TAINT SOURCES:
// - c.GetString("key")
// - c.GetStrings("key")
// - c.GetInt("key")
// - c.Input()
// - c.Ctx.Input.Param(":key")
// - c.Ctx.Input.Query("key")
// - c.Ctx.Input.Header("key")
// - c.Ctx.Input.Cookie("key")
// - c.Ctx.Input.RequestBody
// ===============================================

// GetUser retrieves a user by ID
// GET /api/users/:id
// TAINT SOURCE: c.Ctx.Input.Param(":id")
func (c *UserController) GetUser() {
	// TAINT SOURCE: Path parameter via Beego
	userID := c.Ctx.Input.Param(":id")

	// TAINT PROPAGATION: User input to SQL
	query := fmt.Sprintf("SELECT * FROM users WHERE id = %s", userID)

	// TAINT SINK: SQL injection
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
// TAINT SOURCE: c.GetString("username")
func (c *UserController) GetUserByUsername() {
	// TAINT SOURCE: Query parameter via Beego
	username := c.GetString("username")

	// TAINT PROPAGATION: String concatenation
	query := "SELECT * FROM users WHERE username = '" + username + "'"

	// TAINT SINK: SQL injection
	row := c.db.QueryRow(query)
	var user models.User
	_ = row.Scan(&user.ID, &user.Username, &user.Email)

	c.Data["json"] = user
	c.ServeJSON()
}

// SearchUsers searches users with multiple filters
// GET /api/users/search
// TAINT SOURCE: Multiple c.GetString() calls
func (c *UserController) SearchUsers() {
	// TAINT SOURCES: Multiple query parameters
	searchTerm := c.GetString("q")
	role := c.GetString("role")
	sortBy := c.GetString("sort")
	limit := c.GetString("limit")

	// TAINT PROPAGATION: All params flow to query
	query := fmt.Sprintf(
		"SELECT * FROM users WHERE username LIKE '%%%s%%' AND role = '%s' ORDER BY %s LIMIT %s",
		searchTerm, role, sortBy, limit,
	)

	// TAINT SINK: SQL injection (multiple injection points)
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
// TAINT SOURCE: c.Ctx.Input.RequestBody
func (c *UserController) CreateUser() {
	// TAINT SOURCE: Request body
	var input struct {
		Username string `json:"username"`
		Email    string `json:"email"`
		Role     string `json:"role"`
		Password string `json:"password"`
	}

	// Parse JSON body
	if err := json.Unmarshal(c.Ctx.Input.RequestBody, &input); err != nil {
		c.Data["json"] = map[string]string{"error": "Invalid JSON"}
		c.ServeJSON()
		return
	}

	// TAINT PROPAGATION: Struct fields to SQL
	query := fmt.Sprintf(
		"INSERT INTO users (username, email, role, password) VALUES ('%s', '%s', '%s', '%s')",
		input.Username, input.Email, input.Role, input.Password,
	)

	// TAINT SINK: SQL injection
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
// TAINT SOURCE: c.Ctx.Input.Param + c.Input()
func (c *UserController) UpdateUser() {
	// TAINT SOURCE: Path parameter
	userID := c.Ctx.Input.Param(":id")

	// TAINT SOURCE: Form values via Input()
	email := c.Input().Get("email")
	role := c.Input().Get("role")

	// TAINT PROPAGATION: Multiple sources
	query := fmt.Sprintf(
		"UPDATE users SET email = '%s', role = '%s' WHERE id = %s",
		email, role, userID,
	)

	// TAINT SINK: SQL injection
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
	// TAINT SOURCE: Path parameter
	userID := c.Ctx.Input.Param(":id")

	// TAINT PROPAGATION + SINK
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

// ===============================================
// MULTI-HOP CROSS-FILE TAINT FLOWS
// Handler -> Service -> (SQL/Command/File)
// ===============================================

// CreateUserViaService demonstrates multi-hop flow
// POST /api/users/v2
// TAINT FLOW: Controller -> Service -> Repository
func (c *UserController) CreateUserViaService() {
	var input struct {
		Username string `json:"username"`
		Email    string `json:"email"`
		Role     string `json:"role"`
	}

	// TAINT SOURCE: Request body
	if err := json.Unmarshal(c.Ctx.Input.RequestBody, &input); err != nil {
		c.Data["json"] = map[string]string{"error": "Invalid JSON"}
		c.ServeJSON()
		return
	}

	// TAINT HOP 1: Controller -> Service
	// Input flows to service layer
	user, err := c.userService.CreateUserVulnerable(input.Username, input.Email, input.Role)
	if err != nil {
		c.Data["json"] = map[string]string{"error": err.Error()}
		c.ServeJSON()
		return
	}

	c.Data["json"] = user
	c.ServeJSON()
}

// SearchViaService demonstrates multi-hop search
// GET /api/users/v2/search
func (c *UserController) SearchViaService() {
	// TAINT SOURCE: Query parameter
	searchTerm := c.GetString("q")
	filter := c.GetString("filter")

	// TAINT HOP 1: Controller -> Service
	users, err := c.userService.SearchUsersVulnerable(searchTerm, filter)
	if err != nil {
		c.Data["json"] = map[string]string{"error": err.Error()}
		c.ServeJSON()
		return
	}

	c.Data["json"] = users
	c.ServeJSON()
}

// RunReportViaService demonstrates complex multi-hop
// POST /api/reports
// TAINT FLOW: Controller -> Service -> multiple sinks
func (c *UserController) RunReportViaService() {
	var input struct {
		Name      string `json:"name"`
		Query     string `json:"query"`
		OutputDir string `json:"output_dir"`
	}

	// TAINT SOURCE: Request body
	json.Unmarshal(c.Ctx.Input.RequestBody, &input)

	// TAINT HOP 1: Controller -> Service
	// This flows to both SQL execution AND file operations
	result, err := c.userService.GenerateReport(input.Name, input.Query, input.OutputDir)
	if err != nil {
		c.Data["json"] = map[string]string{"error": err.Error()}
		c.ServeJSON()
		return
	}

	c.Data["json"] = result
	c.ServeJSON()
}

// ===============================================
// HEADER AND COOKIE SOURCES
// ===============================================

// AuthByHeader authenticates via header
// GET /api/auth/header
// TAINT SOURCE: c.Ctx.Input.Header()
func (c *UserController) AuthByHeader() {
	// TAINT SOURCE: HTTP Header
	apiKey := c.Ctx.Input.Header("X-API-Key")
	userAgent := c.Ctx.Input.Header("User-Agent")

	// TAINT PROPAGATION: Header to SQL
	query := fmt.Sprintf("SELECT * FROM users WHERE api_key = '%s'", apiKey)

	// TAINT SINK: SQL injection via header
	row := c.db.QueryRow(query)
	var user models.User
	_ = row.Scan(&user.ID, &user.Username)

	// Also log user agent (taint to log)
	logQuery := fmt.Sprintf("INSERT INTO audit_log (user_agent) VALUES ('%s')", userAgent)
	c.db.Exec(logQuery)

	c.Data["json"] = user
	c.ServeJSON()
}

// AuthByCookie authenticates via cookie
// GET /api/auth/cookie
// TAINT SOURCE: c.Ctx.Input.Cookie()
func (c *UserController) AuthByCookie() {
	// TAINT SOURCE: Cookie
	sessionID := c.Ctx.Input.Cookie("session_id")
	userID := c.Ctx.Input.Cookie("user_id")

	// TAINT PROPAGATION: Cookie to SQL
	query := fmt.Sprintf(
		"SELECT * FROM sessions WHERE id = '%s' AND user_id = %s",
		sessionID, userID,
	)

	// TAINT SINK: SQL injection via cookie
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

// ===============================================
// COMMAND INJECTION
// ===============================================

// RunSystemCommand executes a system command
// POST /api/admin/cmd
// TAINT SOURCE: c.GetString() -> exec.Command
func (c *UserController) RunSystemCommand() {
	// TAINT SOURCE: Query parameters
	cmd := c.GetString("cmd")
	args := c.GetStrings("args")

	// TAINT SINK: Command injection - non-literal command
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

	// TAINT SOURCE: Request body
	json.Unmarshal(c.Ctx.Input.RequestBody, &input)

	// TAINT SINK: Shell command injection
	cmd := exec.Command("sh", "-c", input.Command)
	output, _ := cmd.CombinedOutput()

	c.Data["json"] = map[string]string{"output": string(output)}
	c.ServeJSON()
}

// ===============================================
// PATH TRAVERSAL
// ===============================================

// DownloadFile serves a file
// GET /api/files/download
// TAINT SOURCE: c.GetString("path")
func (c *UserController) DownloadFile() {
	// TAINT SOURCE: Query parameter
	filePath := c.GetString("path")

	// TAINT PROPAGATION: Path construction
	fullPath := filepath.Join("./uploads", filePath)

	// TAINT SINK: Path traversal
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
	// TAINT SOURCE: Form filename
	filename := c.GetString("filename")

	// Get uploaded file
	f, _, err := c.GetFile("file")
	if err != nil {
		c.Data["json"] = map[string]string{"error": err.Error()}
		c.ServeJSON()
		return
	}
	defer f.Close()

	// TAINT PROPAGATION: User-controlled filename
	savePath := filepath.Join("./uploads", filename)

	// TAINT SINK: Arbitrary file write
	c.SaveToFile("file", savePath)

	c.Data["json"] = map[string]string{"saved": savePath}
	c.ServeJSON()
}

// ReadConfig reads configuration file
// GET /api/config/:name
func (c *UserController) ReadConfig() {
	// TAINT SOURCE: Path parameter
	configName := c.Ctx.Input.Param(":name")

	// TAINT PROPAGATION + SINK
	configPath := filepath.Join("./config", configName+".yaml")
	content, err := os.ReadFile(configPath)
	if err != nil {
		c.Data["json"] = map[string]string{"error": "config not found"}
		c.ServeJSON()
		return
	}

	c.Ctx.Output.Body(content)
}

// ===============================================
// SECURE EXAMPLES (for comparison)
// ===============================================

// GetUserSecure uses parameterized query
func (c *UserController) GetUserSecure() {
	userID := c.Ctx.Input.Param(":id")

	// SECURE: Parameterized query
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
