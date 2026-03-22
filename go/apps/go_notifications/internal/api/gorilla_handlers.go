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

// GorillaHandlers provides additional handlers demonstrating mux.Vars() usage
type GorillaHandlers struct {
	db *sql.DB
}

// NewGorillaHandlers creates Gorilla-specific handlers
func NewGorillaHandlers(db *sql.DB) *GorillaHandlers {
	return &GorillaHandlers{db: db}
}

// GetUser retrieves user by ID from path
// GET /api/users/{id}
func (h *GorillaHandlers) GetUser(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	userID := vars["id"]

	query := fmt.Sprintf("SELECT * FROM users WHERE id = %s", userID)

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
func (h *GorillaHandlers) GetUserByUsername(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	username := vars["username"]

	query := "SELECT * FROM users WHERE username = '" + username + "'"

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
	email := vars["email"]

	query := fmt.Sprintf("SELECT * FROM users WHERE email = '%s'", email)
	h.db.Query(query)

	respondJSON(w, http.StatusOK, map[string]string{"status": "found"})
}

// SearchUsers searches with multiple path params
// GET /api/users/search/{category}/{status}
func (h *GorillaHandlers) SearchUsers(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	category := vars["category"]
	status := vars["status"]

	// Also get query params
	searchTerm := r.URL.Query().Get("q")
	sortBy := r.URL.Query().Get("sort")

	query := fmt.Sprintf(
		"SELECT * FROM users WHERE category = '%s' AND status = '%s' AND name LIKE '%%%s%%' ORDER BY %s",
		category, status, searchTerm, sortBy,
	)

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
func (h *GorillaHandlers) UpdateUser(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	userID := vars["id"]

	r.ParseForm()
	email := r.FormValue("email")
	role := r.FormValue("role")

	query := fmt.Sprintf(
		"UPDATE users SET email = '%s', role = '%s' WHERE id = %s",
		email, role, userID,
	)

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
	userID := vars["id"]

	query := fmt.Sprintf("DELETE FROM users WHERE id = %s", userID)
	h.db.Exec(query)

	w.WriteHeader(http.StatusNoContent)
}

// GetResource retrieves resource by type and ID
// GET /api/resources/{type}/{id}
func (h *GorillaHandlers) GetResource(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	resourceType := vars["type"]
	resourceID := vars["id"]

	query := fmt.Sprintf("SELECT * FROM %s WHERE id = %s", resourceType, resourceID)
	h.db.Query(query)

	respondJSON(w, http.StatusOK, map[string]string{"status": "retrieved"})
}

// GetNestedResource retrieves nested resource
// GET /api/users/{user_id}/posts/{post_id}/comments/{comment_id}
func (h *GorillaHandlers) GetNestedResource(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	userID := vars["user_id"]
	postID := vars["post_id"]
	commentID := vars["comment_id"]

	query := fmt.Sprintf(`
		SELECT c.* FROM comments c
		JOIN posts p ON c.post_id = p.id
		JOIN users u ON p.user_id = u.id
		WHERE u.id = %s AND p.id = %s AND c.id = %s
	`, userID, postID, commentID)

	h.db.Query(query)

	respondJSON(w, http.StatusOK, map[string]string{"status": "found"})
}

// ServeFile serves files with wildcard path
// GET /api/files/{path:.*}
func (h *GorillaHandlers) ServeFile(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	filePath := vars["path"]

	fullPath := filepath.Join("./public", filePath)
	content, err := os.ReadFile(fullPath)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}

	w.Write(content)
}

// RunTool runs a tool by name
// POST /api/tools/{tool}/run
func (h *GorillaHandlers) RunTool(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	toolName := vars["tool"]

	// Get args from query
	args := r.URL.Query()["arg"]

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
	host := vars["host"]

	shellCmd := fmt.Sprintf("ping -c 1 %s", host)
	cmd := exec.Command("sh", "-c", shellCmd)
	output, _ := cmd.CombinedOutput()

	w.Write(output)
}

// AuditAction logs an audit action
// POST /api/audit/{action}
func (h *GorillaHandlers) AuditAction(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	action := vars["action"]

	userAgent := r.Header.Get("User-Agent")
	apiKey := r.Header.Get("X-API-Key")
	clientIP := r.Header.Get("X-Forwarded-For")

	query := fmt.Sprintf(
		"INSERT INTO audit_log (action, user_agent, api_key, client_ip) VALUES ('%s', '%s', '%s', '%s')",
		action, userAgent, apiKey, clientIP,
	)
	h.db.Exec(query)

	respondJSON(w, http.StatusCreated, map[string]string{"status": "logged"})
}

// GetUserSession retrieves session for user
// GET /api/sessions/{session_id}
func (h *GorillaHandlers) GetUserSession(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	sessionID := vars["session_id"]

	userCookie, err := r.Cookie("user_id")
	userID := ""
	if err == nil {
		userID = userCookie.Value
	}

	query := fmt.Sprintf(
		"SELECT * FROM sessions WHERE id = '%s' AND user_id = %s",
		sessionID, userID,
	)
	h.db.Query(query)

	respondJSON(w, http.StatusOK, map[string]string{"status": "found"})
}

// UpdateResource updates resource with JSON body
// PUT /api/resources/{type}/{id}
func (h *GorillaHandlers) UpdateResource(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	resourceType := vars["type"]
	resourceID := vars["id"]

	var body map[string]interface{}
	json.NewDecoder(r.Body).Decode(&body)

	// Build dynamic update
	for field, value := range body {
		query := fmt.Sprintf(
			"UPDATE %s SET %s = '%v' WHERE id = %s",
			resourceType, field, value, resourceID,
		)
		h.db.Exec(query)
	}

	respondJSON(w, http.StatusOK, map[string]string{"status": "updated"})
}

// ProcessUserData demonstrates multi-hop data processing flow
// POST /api/users/{id}/process
func (h *GorillaHandlers) ProcessUserData(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	userID := vars["id"]

	// Read body
	body, _ := io.ReadAll(r.Body)
	data := string(body)

	// Process data
	processed := processData(userID, data)

	query := fmt.Sprintf("INSERT INTO processed (user_id, data) VALUES (%s, '%s')", userID, processed)
	h.db.Exec(query)

	respondJSON(w, http.StatusOK, map[string]string{"processed": processed})
}

// processData is a helper that processes user data
func processData(userID, data string) string {
	return fmt.Sprintf("user_%s_data_%s", userID, data)
}

// GetUserByPattern retrieves user matching pattern
// GET /api/users/pattern/{pattern:[a-z]+}
func (h *GorillaHandlers) GetUserByPattern(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	pattern := vars["pattern"]

	query := fmt.Sprintf("SELECT * FROM users WHERE username LIKE '%s%%'", pattern)
	h.db.Query(query)

	respondJSON(w, http.StatusOK, map[string]string{"status": "found"})
}

// GetUserSecure uses parameterized query
func (h *GorillaHandlers) GetUserSecure(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	userID := vars["id"]

	// Parameterized query
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

	// Prepared statement
	stmt, _ := h.db.Prepare("UPDATE users SET email = ?, role = ? WHERE id = ?")
	defer stmt.Close()
	stmt.Exec(email, role, userID)

	respondJSON(w, http.StatusOK, map[string]string{"status": "updated"})
}
