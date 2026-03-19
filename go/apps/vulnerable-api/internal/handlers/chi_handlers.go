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

	"github.com/go-chi/chi/v5"
	"github.com/theauditor/vulnerable-api/internal/repository"
)

// ChiHandler handles HTTP requests using Chi router
type ChiHandler struct {
	userRepo *repository.UserRepository
	db       *sql.DB
}

// NewChiHandler creates a new Chi handler
func NewChiHandler(userRepo *repository.UserRepository, db *sql.DB) *ChiHandler {
	return &ChiHandler{
		userRepo: userRepo,
		db:       db,
	}
}

// ===============================================
// TAINT SOURCE: chi.URLParam, r.URL.Query()
// TAINT SINK: SQL injection
// ===============================================

// GetUser - VULNERABLE: SQL injection via URL parameter
// Taint flow: chi.URLParam(r, "id") -> fmt.Sprintf -> db.Query
func (h *ChiHandler) GetUser(w http.ResponseWriter, r *http.Request) {
	// TAINT SOURCE: URL parameter from Chi
	userID := chi.URLParam(r, "id")

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

// GetUserByUsername - VULNERABLE: SQL injection via URL param
// Taint flow: chi.URLParam -> string concat -> db.QueryRow
func (h *ChiHandler) GetUserByUsername(w http.ResponseWriter, r *http.Request) {
	// TAINT SOURCE: URL parameter
	username := chi.URLParam(r, "username")

	// TAINT PROPAGATION: String concatenation
	query := "SELECT id, username, email FROM users WHERE username = '" + username + "'"

	// TAINT SINK: SQL injection
	row := h.db.QueryRow(query)
	var id int
	var name, email string
	_ = row.Scan(&id, &name, &email)

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"id":       id,
		"username": name,
		"email":    email,
	})
}

// SearchUsers - VULNERABLE: SQL injection via query string
// Taint flow: r.URL.Query().Get("q") -> fmt.Sprintf -> db.Query
func (h *ChiHandler) SearchUsers(w http.ResponseWriter, r *http.Request) {
	// TAINT SOURCE: Query string from URL
	searchTerm := r.URL.Query().Get("q")
	sortBy := r.URL.Query().Get("sort")

	// TAINT PROPAGATION: Multiple user inputs in query
	query := fmt.Sprintf(
		"SELECT * FROM users WHERE username LIKE '%%%s%%' ORDER BY %s",
		searchTerm, sortBy,
	)

	// TAINT SINK: SQL injection (both search and sort vulnerable)
	rows, err := h.db.Query(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]string{"status": "search completed"})
}

// ===============================================
// TAINT SOURCE: r.Body, r.Form, r.PostForm
// TAINT SINK: SQL injection
// ===============================================

// CreateUser - VULNERABLE: SQL injection via request body
// Taint flow: r.Body -> json.Decode -> fmt.Sprintf -> db.Exec
func (h *ChiHandler) CreateUser(w http.ResponseWriter, r *http.Request) {
	var input struct {
		Username string `json:"username"`
		Email    string `json:"email"`
		Role     string `json:"role"`
	}

	// TAINT SOURCE: Request body
	if err := json.NewDecoder(r.Body).Decode(&input); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
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
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w).Encode(map[string]string{"status": "created"})
}

// UpdateUser - VULNERABLE: SQL injection via form data
// Taint flow: r.PostFormValue -> fmt.Sprintf -> db.Exec
func (h *ChiHandler) UpdateUser(w http.ResponseWriter, r *http.Request) {
	// TAINT SOURCE: URL parameter
	userID := chi.URLParam(r, "id")

	// Parse form data
	r.ParseForm()

	// TAINT SOURCE: Form values
	email := r.PostFormValue("email")
	role := r.PostFormValue("role")

	// TAINT PROPAGATION: Multiple sources combine
	query := fmt.Sprintf(
		"UPDATE users SET email = '%s', role = '%s' WHERE id = %s",
		email, role, userID,
	)

	// TAINT SINK: SQL injection from multiple sources
	_, err := h.db.Exec(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	json.NewEncoder(w).Encode(map[string]string{"status": "updated"})
}

// DeleteUser - VULNERABLE: SQL injection via DELETE
func (h *ChiHandler) DeleteUser(w http.ResponseWriter, r *http.Request) {
	// TAINT SOURCE: URL parameter
	userID := chi.URLParam(r, "id")

	// TAINT PROPAGATION: Direct to query
	query := fmt.Sprintf("DELETE FROM users WHERE id = %s", userID)

	// TAINT SINK: SQL injection
	_, err := h.db.Exec(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusNoContent)
}

// ===============================================
// TAINT SINK: Command injection
// ===============================================

// RunCommand - VULNERABLE: Command injection
// Taint flow: r.URL.Query() -> exec.Command
func (h *ChiHandler) RunCommand(w http.ResponseWriter, r *http.Request) {
	// TAINT SOURCE: Query parameters
	cmd := r.URL.Query().Get("cmd")
	args := r.URL.Query().Get("args")

	// TAINT SINK: Command injection - non-literal command
	command := exec.Command(cmd, args)
	output, err := command.CombinedOutput()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "text/plain")
	w.Write(output)
}

// SystemInfo - VULNERABLE: Command injection via header
func (h *ChiHandler) SystemInfo(w http.ResponseWriter, r *http.Request) {
	// TAINT SOURCE: HTTP header
	infoType := r.Header.Get("X-Info-Type")

	// TAINT PROPAGATION: Command construction
	shellCmd := fmt.Sprintf("uname -%s", infoType)

	// TAINT SINK: Shell command injection
	cmd := exec.Command("sh", "-c", shellCmd)
	output, _ := cmd.Output()

	w.Write(output)
}

// ===============================================
// TAINT SINK: Path traversal
// ===============================================

// ServeFile - VULNERABLE: Path traversal via URL param
// Taint flow: chi.URLParam -> filepath.Join -> os.Open
func (h *ChiHandler) ServeFile(w http.ResponseWriter, r *http.Request) {
	// TAINT SOURCE: URL parameter via Chi wildcard
	filename := chi.URLParam(r, "*")

	// TAINT PROPAGATION: Path construction
	filePath := filepath.Join("./public", filename)

	// TAINT SINK: Path traversal
	file, err := os.Open(filePath)
	if err != nil {
		http.Error(w, "file not found", http.StatusNotFound)
		return
	}
	defer file.Close()

	io.Copy(w, file)
}

// UploadFile - VULNERABLE: Path traversal on file upload
func (h *ChiHandler) UploadFile(w http.ResponseWriter, r *http.Request) {
	// Parse multipart form
	r.ParseMultipartForm(10 << 20) // 10MB max

	// TAINT SOURCE: Form field for filename
	filename := r.FormValue("filename")

	// Get the file from form
	file, _, err := r.FormFile("file")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer file.Close()

	// TAINT PROPAGATION: User-controlled filename
	savePath := filepath.Join("/uploads", filename)

	// TAINT SINK: Path traversal - arbitrary file write
	dst, err := os.Create(savePath)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer dst.Close()

	io.Copy(dst, file)

	json.NewEncoder(w).Encode(map[string]string{"status": "uploaded"})
}

// ===============================================
// TAINT SOURCE: r.Context() derived values
// ===============================================

// AuditLog - VULNERABLE: Context value to SQL
func (h *ChiHandler) AuditLog(w http.ResponseWriter, r *http.Request) {
	// TAINT SOURCE: Value from context (potentially set from header/cookie)
	userID := r.Context().Value("user_id").(string)
	action := chi.URLParam(r, "action")

	// TAINT PROPAGATION: Context value + URL param
	query := fmt.Sprintf(
		"INSERT INTO audit_logs (user_id, action) VALUES (%s, '%s')",
		userID, action,
	)

	// TAINT SINK: SQL injection
	_, err := h.db.Exec(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
}

// ===============================================
// SECURE: Parameterized queries for comparison
// ===============================================

// GetUserSecure - SECURE: Uses parameterized query
func (h *ChiHandler) GetUserSecure(w http.ResponseWriter, r *http.Request) {
	userID := chi.URLParam(r, "id")

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

// CreateUserSecure - SECURE: Uses Prepare statement
func (h *ChiHandler) CreateUserSecure(w http.ResponseWriter, r *http.Request) {
	var input struct {
		Username string `json:"username"`
		Email    string `json:"email"`
	}

	if err := json.NewDecoder(r.Body).Decode(&input); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	// SECURE: Prepared statement
	stmt, err := h.db.Prepare("INSERT INTO users (username, email) VALUES (?, ?)")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer stmt.Close()

	_, err = stmt.Exec(input.Username, input.Email)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
}
