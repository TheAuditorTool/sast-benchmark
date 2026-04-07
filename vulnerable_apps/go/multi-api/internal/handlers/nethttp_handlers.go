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

	"github.com/theauditor/multi-api/internal/repository"
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

// GetUser handles user lookup by ID
func (h *NetHTTPHandler) GetUser(w http.ResponseWriter, r *http.Request) {
	userID := r.URL.Query().Get("id")

	query := fmt.Sprintf("SELECT * FROM users WHERE id = %s", userID)

	rows, err := h.db.Query(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]string{"status": "success"})
}

// SearchUsers searches users with ordering
func (h *NetHTTPHandler) SearchUsers(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()

	searchTerm := r.Form.Get("q")
	orderBy := r.Form.Get("order")

	query := fmt.Sprintf(
		"SELECT * FROM users WHERE username LIKE '%%%s%%' ORDER BY %s",
		searchTerm, orderBy,
	)

	rows, err := h.db.Query(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]string{"status": "search completed"})
}

// CreateUser creates a new user from form data
func (h *NetHTTPHandler) CreateUser(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()

	username := r.FormValue("username")
	email := r.FormValue("email")
	role := r.FormValue("role")

	query := fmt.Sprintf(
		"INSERT INTO users (username, email, role) VALUES ('%s', '%s', '%s')",
		username, email, role,
	)

	_, err := h.db.Exec(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w).Encode(map[string]string{"status": "created"})
}

// UpdateUser updates a user's email and role
func (h *NetHTTPHandler) UpdateUser(w http.ResponseWriter, r *http.Request) {
	userID := r.URL.Query().Get("id")

	r.ParseForm()

	email := r.PostFormValue("email")
	role := r.PostFormValue("role")

	query := fmt.Sprintf(
		"UPDATE users SET email = '%s', role = '%s' WHERE id = %s",
		email, role, userID,
	)

	_, err := h.db.Exec(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	json.NewEncoder(w).Encode(map[string]string{"status": "updated"})
}

// ProcessJSON processes a JSON body and inserts data
func (h *NetHTTPHandler) ProcessJSON(w http.ResponseWriter, r *http.Request) {
	var input struct {
		TableName string `json:"table"`
		Column    string `json:"column"`
		Value     string `json:"value"`
	}

	if err := json.NewDecoder(r.Body).Decode(&input); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	query := fmt.Sprintf(
		"INSERT INTO %s (%s) VALUES ('%s')",
		input.TableName, input.Column, input.Value,
	)

	_, err := h.db.Exec(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
}

// RawBodyToSQL inserts the raw request body into the database
func (h *NetHTTPHandler) RawBodyToSQL(w http.ResponseWriter, r *http.Request) {
	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	query := fmt.Sprintf("INSERT INTO logs (raw_data) VALUES ('%s')", string(body))

	_, err = h.db.Exec(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
}

// RunCommand executes a command from query parameters
func (h *NetHTTPHandler) RunCommand(w http.ResponseWriter, r *http.Request) {
	cmd := r.URL.Query().Get("cmd")
	args := r.URL.Query().Get("args")

	command := exec.Command(cmd, args)
	output, err := command.CombinedOutput()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Write(output)
}

// ShellExec executes a shell command from form data
func (h *NetHTTPHandler) ShellExec(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()

	shellCmd := r.FormValue("command")

	cmd := exec.Command("sh", "-c", shellCmd)
	output, _ := cmd.CombinedOutput()

	w.Write(output)
}

// ProcessFile counts lines in a file
func (h *NetHTTPHandler) ProcessFile(w http.ResponseWriter, r *http.Request) {
	filename := r.URL.Query().Get("file")

	shellCmd := fmt.Sprintf("cat %s | wc -l", filename)

	cmd := exec.Command("sh", "-c", shellCmd)
	output, _ := cmd.Output()

	w.Write(output)
}

// ServeFile serves a file from the static directory
func (h *NetHTTPHandler) ServeFile(w http.ResponseWriter, r *http.Request) {
	filename := r.URL.Query().Get("file")

	filePath := filepath.Join("./static", filename)

	file, err := os.Open(filePath)
	if err != nil {
		http.Error(w, "file not found", http.StatusNotFound)
		return
	}
	defer file.Close()

	io.Copy(w, file)
}

// ReadFile reads a file from the URL path
func (h *NetHTTPHandler) ReadFile(w http.ResponseWriter, r *http.Request) {
	filePath := r.URL.Path[len("/files/"):]

	content, err := os.ReadFile(filePath)
	if err != nil {
		http.Error(w, err.Error(), http.StatusNotFound)
		return
	}

	w.Write(content)
}

// WriteFile writes request body to a file
func (h *NetHTTPHandler) WriteFile(w http.ResponseWriter, r *http.Request) {
	filePath := r.URL.Query().Get("path")

	content, _ := io.ReadAll(r.Body)

	err := os.WriteFile(filePath, content, 0644)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
}

// LogRequest logs request headers to the database
func (h *NetHTTPHandler) LogRequest(w http.ResponseWriter, r *http.Request) {
	userAgent := r.Header.Get("User-Agent")
	referer := r.Header.Get("Referer")
	customData := r.Header.Get("X-Custom-Data")

	query := fmt.Sprintf(
		"INSERT INTO access_logs (user_agent, referer, custom_data) VALUES ('%s', '%s', '%s')",
		userAgent, referer, customData,
	)

	_, err := h.db.Exec(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
}

// ProcessCookie queries session data from a cookie
func (h *NetHTTPHandler) ProcessCookie(w http.ResponseWriter, r *http.Request) {
	cookie, err := r.Cookie("session_data")
	if err != nil {
		http.Error(w, "no cookie", http.StatusBadRequest)
		return
	}

	query := fmt.Sprintf("SELECT * FROM sessions WHERE data = '%s'", cookie.Value)

	rows, err := h.db.Query(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	w.WriteHeader(http.StatusOK)
}

// ComplexHandler processes a multi-step insert
func (h *NetHTTPHandler) ComplexHandler(w http.ResponseWriter, r *http.Request) {
	table := r.URL.Query().Get("table")
	field := r.URL.Query().Get("field")
	value := r.URL.Query().Get("value")

	sanitizedTable := sanitizeTableName(table)
	sanitizedField := sanitizeFieldName(field)

	query := buildInsertQuery(sanitizedTable, sanitizedField, value)

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

// GetUserAlt looks up a user by ID with parameterized query
func (h *NetHTTPHandler) GetUserAlt(w http.ResponseWriter, r *http.Request) {
	userID := r.URL.Query().Get("id")

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

// CreateUserAlt creates a user with prepared statement
func (h *NetHTTPHandler) CreateUserAlt(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()
	username := r.FormValue("username")
	email := r.FormValue("email")

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
