package handlers

import (
	"database/sql"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"

	"github.com/theauditor/vulnerable-api/internal/repository"
)

// NetHTTPHandler handles HTTP requests using standard net/http
type NetHTTPHandler struct {
	userRepo *repository.UserRepository
	db       *sql.DB
}

// NewNetHTTPHandler creates a new net/http handler
func NewNetHTTPHandler(userRepo *repository.UserRepository, db *sql.DB) *NetHTTPHandler {
	return &NetHTTPHandler{
		userRepo: userRepo,
		db:       db,
	}
}

// ===============================================
// TAINT SOURCE: r.URL.Query(), r.FormValue, r.Form
// TAINT SINK: SQL injection
// ===============================================

// GetUser - VULNERABLE: SQL injection via URL query
// Taint flow: r.URL.Query().Get("id") -> fmt.Sprintf -> db.Query
func (h *NetHTTPHandler) GetUser(w http.ResponseWriter, r *http.Request) {
	// TAINT SOURCE: URL query parameter
	userID := r.URL.Query().Get("id")

	// TAINT PROPAGATION: User input flows into SQL query
	query := fmt.Sprintf("SELECT * FROM users WHERE id = %s", userID)

	// TAINT SINK: SQL injection
	rows, err := h.db.Query(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]string{"status": "success"})
}

// SearchUsers - VULNERABLE: SQL injection via r.Form
// Taint flow: r.Form.Get("q") -> fmt.Sprintf -> db.Query
func (h *NetHTTPHandler) SearchUsers(w http.ResponseWriter, r *http.Request) {
	// Parse the form
	r.ParseForm()

	// TAINT SOURCE: Form data via r.Form
	searchTerm := r.Form.Get("q")
	orderBy := r.Form.Get("order")

	// TAINT PROPAGATION: Multiple form values in query
	query := fmt.Sprintf(
		"SELECT * FROM users WHERE username LIKE '%%%s%%' ORDER BY %s",
		searchTerm, orderBy,
	)

	// TAINT SINK: SQL injection
	rows, err := h.db.Query(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]string{"status": "search completed"})
}

// CreateUser - VULNERABLE: SQL injection via r.FormValue
// Taint flow: r.FormValue -> fmt.Sprintf -> db.Exec
func (h *NetHTTPHandler) CreateUser(w http.ResponseWriter, r *http.Request) {
	// Parse the form
	r.ParseForm()

	// TAINT SOURCE: Form values
	username := r.FormValue("username")
	email := r.FormValue("email")
	role := r.FormValue("role")

	// TAINT PROPAGATION: Form values flow to query
	query := fmt.Sprintf(
		"INSERT INTO users (username, email, role) VALUES ('%s', '%s', '%s')",
		username, email, role,
	)

	// TAINT SINK: SQL injection
	_, err := h.db.Exec(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w).Encode(map[string]string{"status": "created"})
}

// UpdateUser - VULNERABLE: SQL injection via r.PostFormValue
// Taint flow: r.PostFormValue -> fmt.Sprintf -> db.Exec
func (h *NetHTTPHandler) UpdateUser(w http.ResponseWriter, r *http.Request) {
	// TAINT SOURCE: URL query for ID
	userID := r.URL.Query().Get("id")

	// Parse POST form
	r.ParseForm()

	// TAINT SOURCE: POST form values
	email := r.PostFormValue("email")
	role := r.PostFormValue("role")

	// TAINT PROPAGATION: Multiple sources combine
	query := fmt.Sprintf(
		"UPDATE users SET email = '%s', role = '%s' WHERE id = %s",
		email, role, userID,
	)

	// TAINT SINK: SQL injection
	_, err := h.db.Exec(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	json.NewEncoder(w).Encode(map[string]string{"status": "updated"})
}

// ===============================================
// TAINT SOURCE: r.Body - raw request body
// TAINT SINK: SQL injection
// ===============================================

// ProcessJSON - VULNERABLE: SQL injection via JSON body
// Taint flow: r.Body -> json.Decode -> fmt.Sprintf -> db.Exec
func (h *NetHTTPHandler) ProcessJSON(w http.ResponseWriter, r *http.Request) {
	var input struct {
		TableName string `json:"table"`
		Column    string `json:"column"`
		Value     string `json:"value"`
	}

	// TAINT SOURCE: Request body
	if err := json.NewDecoder(r.Body).Decode(&input); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	// TAINT PROPAGATION: Struct fields flow to query
	// CRITICAL: Table and column names from user input!
	query := fmt.Sprintf(
		"INSERT INTO %s (%s) VALUES ('%s')",
		input.TableName, input.Column, input.Value,
	)

	// TAINT SINK: SQL injection - table/column injection
	_, err := h.db.Exec(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
}

// RawBodyToSQL - VULNERABLE: Raw body directly to SQL
func (h *NetHTTPHandler) RawBodyToSQL(w http.ResponseWriter, r *http.Request) {
	// TAINT SOURCE: Raw request body
	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	// TAINT PROPAGATION: Direct body to query
	query := fmt.Sprintf("INSERT INTO logs (raw_data) VALUES ('%s')", string(body))

	// TAINT SINK: SQL injection
	_, err = h.db.Exec(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
}

// ===============================================
// TAINT SINK: Command injection
// ===============================================

// RunCommand - VULNERABLE: Command injection via query params
// Taint flow: r.URL.Query() -> exec.Command
func (h *NetHTTPHandler) RunCommand(w http.ResponseWriter, r *http.Request) {
	// TAINT SOURCE: Query parameters
	cmd := r.URL.Query().Get("cmd")
	args := r.URL.Query().Get("args")

	// TAINT SINK: Command injection
	command := exec.Command(cmd, args)
	output, err := command.CombinedOutput()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Write(output)
}

// ShellExec - VULNERABLE: Shell command injection via form
func (h *NetHTTPHandler) ShellExec(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()

	// TAINT SOURCE: Form value
	shellCmd := r.FormValue("command")

	// TAINT SINK: Shell command injection
	cmd := exec.Command("sh", "-c", shellCmd)
	output, _ := cmd.CombinedOutput()

	w.Write(output)
}

// ProcessFile - VULNERABLE: Command injection via filename
func (h *NetHTTPHandler) ProcessFile(w http.ResponseWriter, r *http.Request) {
	// TAINT SOURCE: Query parameter
	filename := r.URL.Query().Get("file")

	// TAINT PROPAGATION: Shell command construction
	shellCmd := fmt.Sprintf("cat %s | wc -l", filename)

	// TAINT SINK: Command injection
	cmd := exec.Command("sh", "-c", shellCmd)
	output, _ := cmd.Output()

	w.Write(output)
}

// ===============================================
// TAINT SINK: Path traversal
// ===============================================

// ServeFile - VULNERABLE: Path traversal via query parameter
// Taint flow: r.URL.Query() -> filepath.Join -> os.Open
func (h *NetHTTPHandler) ServeFile(w http.ResponseWriter, r *http.Request) {
	// TAINT SOURCE: Query parameter
	filename := r.URL.Query().Get("file")

	// TAINT PROPAGATION: Path construction
	filePath := filepath.Join("./static", filename)

	// TAINT SINK: Path traversal
	file, err := os.Open(filePath)
	if err != nil {
		http.Error(w, "file not found", http.StatusNotFound)
		return
	}
	defer file.Close()

	io.Copy(w, file)
}

// ReadFile - VULNERABLE: Path traversal via URL path
func (h *NetHTTPHandler) ReadFile(w http.ResponseWriter, r *http.Request) {
	// TAINT SOURCE: URL path
	filePath := r.URL.Path[len("/files/"):]

	// TAINT SINK: Path traversal - direct use of URL path
	content, err := os.ReadFile(filePath)
	if err != nil {
		http.Error(w, err.Error(), http.StatusNotFound)
		return
	}

	w.Write(content)
}

// WriteFile - VULNERABLE: Path traversal on write
func (h *NetHTTPHandler) WriteFile(w http.ResponseWriter, r *http.Request) {
	// TAINT SOURCE: Query parameter for path
	filePath := r.URL.Query().Get("path")

	// Read body
	content, _ := io.ReadAll(r.Body)

	// TAINT SINK: Path traversal - arbitrary file write
	err := os.WriteFile(filePath, content, 0644)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
}

// ===============================================
// TAINT SOURCE: HTTP Headers and Cookies
// ===============================================

// LogRequest - VULNERABLE: Header injection to SQL
func (h *NetHTTPHandler) LogRequest(w http.ResponseWriter, r *http.Request) {
	// TAINT SOURCE: HTTP headers
	userAgent := r.Header.Get("User-Agent")
	referer := r.Header.Get("Referer")
	customData := r.Header.Get("X-Custom-Data")

	// TAINT PROPAGATION: Headers to query
	query := fmt.Sprintf(
		"INSERT INTO access_logs (user_agent, referer, custom_data) VALUES ('%s', '%s', '%s')",
		userAgent, referer, customData,
	)

	// TAINT SINK: SQL injection via headers
	_, err := h.db.Exec(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
}

// ProcessCookie - VULNERABLE: Cookie value to SQL
func (h *NetHTTPHandler) ProcessCookie(w http.ResponseWriter, r *http.Request) {
	// TAINT SOURCE: Cookie value
	cookie, err := r.Cookie("session_data")
	if err != nil {
		http.Error(w, "no cookie", http.StatusBadRequest)
		return
	}

	// TAINT PROPAGATION: Cookie value to query
	query := fmt.Sprintf("SELECT * FROM sessions WHERE data = '%s'", cookie.Value)

	// TAINT SINK: SQL injection via cookie
	rows, err := h.db.Query(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	w.WriteHeader(http.StatusOK)
}

// ===============================================
// COMPLEX FLOW: Multi-step propagation
// ===============================================

// ComplexHandler - VULNERABLE: Multi-step taint flow
func (h *NetHTTPHandler) ComplexHandler(w http.ResponseWriter, r *http.Request) {
	// TAINT SOURCE: Multiple query parameters
	table := r.URL.Query().Get("table")
	field := r.URL.Query().Get("field")
	value := r.URL.Query().Get("value")

	// TAINT PROPAGATION: Step 1 - Variable assignments
	sanitizedTable := sanitizeTableName(table)  // Not actually sanitized!
	sanitizedField := sanitizeFieldName(field)  // Not actually sanitized!

	// TAINT PROPAGATION: Step 2 - Query construction
	query := buildInsertQuery(sanitizedTable, sanitizedField, value)

	// TAINT SINK: SQL injection
	_, err := h.db.Exec(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
}

// sanitizeTableName - FAKE sanitizer that doesn't sanitize
func sanitizeTableName(table string) string {
	// This looks like sanitization but does nothing
	return table
}

// sanitizeFieldName - FAKE sanitizer that doesn't sanitize
func sanitizeFieldName(field string) string {
	// This looks like sanitization but does nothing
	return field
}

// buildInsertQuery - Helper that propagates taint
func buildInsertQuery(table, field, value string) string {
	// Taint flows through and creates vulnerable query
	return fmt.Sprintf("INSERT INTO %s (%s) VALUES ('%s')", table, field, value)
}

// ===============================================
// SECURE: Parameterized queries for comparison
// ===============================================

// GetUserSecure - SECURE: Uses parameterized query
func (h *NetHTTPHandler) GetUserSecure(w http.ResponseWriter, r *http.Request) {
	userID := r.URL.Query().Get("id")

	// SECURE: Parameterized query
	query := "SELECT * FROM users WHERE id = ?"
	rows, err := h.db.Query(query, userID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]string{"status": "success"})
}

// CreateUserSecure - SECURE: Uses Prepare + Exec
func (h *NetHTTPHandler) CreateUserSecure(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()
	username := r.FormValue("username")
	email := r.FormValue("email")

	// SECURE: Prepared statement
	stmt, err := h.db.Prepare("INSERT INTO users (username, email) VALUES (?, ?)")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer stmt.Close()

	_, err = stmt.Exec(username, email)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
}
