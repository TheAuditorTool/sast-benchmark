package api

import (
	"database/sql"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"

	"github.com/gorilla/mux"
)

// GorillaHandlers provides additional handlers demonstrating mux.Vars() taint sources
// This file focuses on Gorilla Mux specific taint patterns
type GorillaHandlers struct {
	db *sql.DB
}

// NewGorillaHandlers creates Gorilla-specific handlers
func NewGorillaHandlers(db *sql.DB) *GorillaHandlers {
	return &GorillaHandlers{db: db}
}

// ===============================================
// GORILLA MUX TAINT SOURCES:
// - mux.Vars(r)["key"] - Path parameters
// - r.URL.Query().Get("key") - Query parameters
// - r.Header.Get("key") - Headers
// - r.Cookie("name") - Cookies
// - r.FormValue("key") - Form data
// - r.Body - Request body
// ===============================================

// GetUser retrieves user by ID from path
// GET /api/users/{id}
// TAINT SOURCE: mux.Vars(r)["id"]
func (h *GorillaHandlers) GetUser(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	// TAINT SOURCE: Path parameter via Gorilla mux
	userID := vars["id"]

	// TAINT PROPAGATION: Path param to SQL
	query := fmt.Sprintf("SELECT * FROM users WHERE id = %s", userID)

	// TAINT SINK: SQL injection
	rows, err := h.db.Query(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	respondJSON(w, http.StatusOK, map[string]string{"status": "success"})
}

// GetUserByUsername retrieves user by username
// GET /api/users/by-name/{username}
// TAINT SOURCE: mux.Vars(r)["username"]
func (h *GorillaHandlers) GetUserByUsername(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	// TAINT SOURCE: Path parameter
	username := vars["username"]

	// TAINT PROPAGATION: String concatenation
	query := "SELECT * FROM users WHERE username = '" + username + "'"

	// TAINT SINK: SQL injection
	row := h.db.QueryRow(query)
	var id int
	var name, email string
	_ = row.Scan(&id, &name, &email)

	respondJSON(w, http.StatusOK, map[string]interface{}{
		"id":       id,
		"username": name,
		"email":    email,
	})
}

// GetUserByEmail retrieves user by email
// GET /api/users/by-email/{email}
func (h *GorillaHandlers) GetUserByEmail(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	// TAINT SOURCE: Path parameter
	email := vars["email"]

	// TAINT SINK: SQL injection
	query := fmt.Sprintf("SELECT * FROM users WHERE email = '%s'", email)
	h.db.Query(query)

	respondJSON(w, http.StatusOK, map[string]string{"status": "found"})
}

// SearchUsers searches with multiple path params
// GET /api/users/search/{category}/{status}
// TAINT SOURCES: Multiple mux.Vars
func (h *GorillaHandlers) SearchUsers(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	// TAINT SOURCES: Multiple path parameters
	category := vars["category"]
	status := vars["status"]

	// Also get query params
	searchTerm := r.URL.Query().Get("q")
	sortBy := r.URL.Query().Get("sort")

	// TAINT PROPAGATION: All sources in query
	query := fmt.Sprintf(
		"SELECT * FROM users WHERE category = '%s' AND status = '%s' AND name LIKE '%%%s%%' ORDER BY %s",
		category, status, searchTerm, sortBy,
	)

	// TAINT SINK: SQL injection (4 points)
	rows, err := h.db.Query(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	respondJSON(w, http.StatusOK, map[string]string{"status": "searched"})
}

// UpdateUser updates user by ID
// PUT /api/users/{id}
// TAINT SOURCES: mux.Vars + form data
func (h *GorillaHandlers) UpdateUser(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	// TAINT SOURCE: Path parameter
	userID := vars["id"]

	// TAINT SOURCE: Form data
	r.ParseForm()
	email := r.FormValue("email")
	role := r.FormValue("role")

	// TAINT PROPAGATION: Multiple sources
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

	respondJSON(w, http.StatusOK, map[string]string{"status": "updated"})
}

// DeleteUser deletes user by ID
// DELETE /api/users/{id}
func (h *GorillaHandlers) DeleteUser(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	// TAINT SOURCE
	userID := vars["id"]

	// TAINT SINK
	query := fmt.Sprintf("DELETE FROM users WHERE id = %s", userID)
	h.db.Exec(query)

	w.WriteHeader(http.StatusNoContent)
}

// ===============================================
// PATH PARAMETERS WITH SPECIAL CHARACTERS
// ===============================================

// GetResource retrieves resource by type and ID
// GET /api/resources/{type}/{id}
// Tests path params with special characters
func (h *GorillaHandlers) GetResource(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	// TAINT SOURCES: Multiple path params
	resourceType := vars["type"]
	resourceID := vars["id"]

	// TAINT SINK: Table name + ID injection
	query := fmt.Sprintf("SELECT * FROM %s WHERE id = %s", resourceType, resourceID)
	h.db.Query(query)

	respondJSON(w, http.StatusOK, map[string]string{"status": "retrieved"})
}

// GetNestedResource retrieves nested resource
// GET /api/users/{user_id}/posts/{post_id}/comments/{comment_id}
// Tests deep nesting
func (h *GorillaHandlers) GetNestedResource(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	// TAINT SOURCES: Three path parameters
	userID := vars["user_id"]
	postID := vars["post_id"]
	commentID := vars["comment_id"]

	// TAINT SINK: Multiple joins with tainted params
	query := fmt.Sprintf(`
		SELECT c.* FROM comments c
		JOIN posts p ON c.post_id = p.id
		JOIN users u ON p.user_id = u.id
		WHERE u.id = %s AND p.id = %s AND c.id = %s
	`, userID, postID, commentID)

	h.db.Query(query)

	respondJSON(w, http.StatusOK, map[string]string{"status": "found"})
}

// ===============================================
// WILDCARD PATH PARAMETERS
// ===============================================

// ServeFile serves files with wildcard path
// GET /api/files/{path:.*}
// TAINT SOURCE: Wildcard path parameter
func (h *GorillaHandlers) ServeFile(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	// TAINT SOURCE: Wildcard path (can contain /)
	filePath := vars["path"]

	// TAINT SINK: Path traversal
	fullPath := filepath.Join("./public", filePath)
	content, err := os.ReadFile(fullPath)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}

	w.Write(content)
}

// ===============================================
// COMMAND INJECTION VIA PATH PARAMS
// ===============================================

// RunTool runs a tool by name
// POST /api/tools/{tool}/run
// TAINT SOURCE: mux.Vars -> exec.Command
func (h *GorillaHandlers) RunTool(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	// TAINT SOURCE: Tool name from path
	toolName := vars["tool"]

	// Get args from query
	args := r.URL.Query()["arg"]

	// TAINT SINK: Command injection
	cmd := exec.Command(toolName, args...)
	output, err := cmd.CombinedOutput()
	if err != nil {
		respondJSON(w, http.StatusInternalServerError, map[string]string{"error": err.Error()})
		return
	}

	respondJSON(w, http.StatusOK, map[string]string{"output": string(output)})
}

// PingHost pings a host
// GET /api/network/ping/{host}
func (h *GorillaHandlers) PingHost(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	// TAINT SOURCE
	host := vars["host"]

	// TAINT SINK: Shell command injection
	shellCmd := fmt.Sprintf("ping -c 1 %s", host)
	cmd := exec.Command("sh", "-c", shellCmd)
	output, _ := cmd.CombinedOutput()

	w.Write(output)
}

// ===============================================
// HEADER TAINT + PATH PARAM
// ===============================================

// AuditAction logs an audit action
// POST /api/audit/{action}
// TAINT SOURCES: mux.Vars + headers
func (h *GorillaHandlers) AuditAction(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	// TAINT SOURCE: Path parameter
	action := vars["action"]

	// TAINT SOURCE: Headers
	userAgent := r.Header.Get("User-Agent")
	apiKey := r.Header.Get("X-API-Key")
	clientIP := r.Header.Get("X-Forwarded-For")

	// TAINT SINK: SQL injection from path + headers
	query := fmt.Sprintf(
		"INSERT INTO audit_log (action, user_agent, api_key, client_ip) VALUES ('%s', '%s', '%s', '%s')",
		action, userAgent, apiKey, clientIP,
	)
	h.db.Exec(query)

	respondJSON(w, http.StatusCreated, map[string]string{"status": "logged"})
}

// ===============================================
// COOKIE TAINT + PATH PARAM
// ===============================================

// GetUserSession retrieves session for user
// GET /api/sessions/{session_id}
// TAINT SOURCES: mux.Vars + cookie
func (h *GorillaHandlers) GetUserSession(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	// TAINT SOURCE: Path parameter
	sessionID := vars["session_id"]

	// TAINT SOURCE: Cookie
	userCookie, err := r.Cookie("user_id")
	userID := ""
	if err == nil {
		userID = userCookie.Value
	}

	// TAINT SINK: SQL injection from path + cookie
	query := fmt.Sprintf(
		"SELECT * FROM sessions WHERE id = '%s' AND user_id = %s",
		sessionID, userID,
	)
	h.db.Query(query)

	respondJSON(w, http.StatusOK, map[string]string{"status": "found"})
}

// ===============================================
// REQUEST BODY + PATH PARAM
// ===============================================

// UpdateResource updates resource with JSON body
// PUT /api/resources/{type}/{id}
// TAINT SOURCES: mux.Vars + r.Body
func (h *GorillaHandlers) UpdateResource(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	// TAINT SOURCES: Path parameters
	resourceType := vars["type"]
	resourceID := vars["id"]

	// TAINT SOURCE: Request body
	var body map[string]interface{}
	json.NewDecoder(r.Body).Decode(&body)

	// Build dynamic update
	for field, value := range body {
		// TAINT SINK: SQL injection (table, column, value, ID)
		query := fmt.Sprintf(
			"UPDATE %s SET %s = '%v' WHERE id = %s",
			resourceType, field, value, resourceID,
		)
		h.db.Exec(query)
	}

	respondJSON(w, http.StatusOK, map[string]string{"status": "updated"})
}

// ===============================================
// MULTI-HOP WITH PATH PARAMS
// ===============================================

// ProcessUserData demonstrates multi-hop flow
// POST /api/users/{id}/process
func (h *GorillaHandlers) ProcessUserData(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	// HOP 1 TAINT SOURCE: Path parameter
	userID := vars["id"]

	// HOP 1: Read body
	body, _ := io.ReadAll(r.Body)
	data := string(body)

	// HOP 2: Process (taint propagates)
	processed := processData(userID, data)

	// HOP 2 TAINT SINK: SQL with processed data
	query := fmt.Sprintf("INSERT INTO processed (user_id, data) VALUES (%s, '%s')", userID, processed)
	h.db.Exec(query)

	respondJSON(w, http.StatusOK, map[string]string{"processed": processed})
}

// processData is a helper that propagates taint
func processData(userID, data string) string {
	// TAINT PROPAGATION: Both inputs flow to output
	return fmt.Sprintf("user_%s_data_%s", userID, data)
}

// ===============================================
// REGEX PATH PARAMETERS
// ===============================================

// GetUserByPattern retrieves user matching pattern
// GET /api/users/pattern/{pattern:[a-z]+}
func (h *GorillaHandlers) GetUserByPattern(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	// TAINT SOURCE: Regex-constrained but still taintable
	// Note: Regex only validates format, not content safety
	pattern := vars["pattern"]

	// TAINT SINK: SQL injection (regex doesn't prevent injection)
	query := fmt.Sprintf("SELECT * FROM users WHERE username LIKE '%s%%'", pattern)
	h.db.Query(query)

	respondJSON(w, http.StatusOK, map[string]string{"status": "found"})
}

// ===============================================
// SECURE EXAMPLES
// ===============================================

// GetUserSecure uses parameterized query
func (h *GorillaHandlers) GetUserSecure(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	userID := vars["id"]

	// SECURE: Parameterized query
	query := "SELECT * FROM users WHERE id = ?"
	rows, err := h.db.Query(query, userID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	respondJSON(w, http.StatusOK, map[string]string{"status": "success"})
}

// UpdateUserSecure uses prepared statement
func (h *GorillaHandlers) UpdateUserSecure(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	userID := vars["id"]

	r.ParseForm()
	email := r.FormValue("email")
	role := r.FormValue("role")

	// SECURE: Prepared statement
	stmt, _ := h.db.Prepare("UPDATE users SET email = ?, role = ? WHERE id = ?")
	defer stmt.Close()
	stmt.Exec(email, role, userID)

	respondJSON(w, http.StatusOK, map[string]string{"status": "updated"})
}
