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
	"github.com/theauditor/multi-api/internal/repository"
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

// GetUser handles user lookup by ID via Chi
func (h *ChiHandler) GetUser(w http.ResponseWriter, r *http.Request) {
	userID := chi.URLParam(r, "id")

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

// GetUserByUsername looks up a user by username via Chi
func (h *ChiHandler) GetUserByUsername(w http.ResponseWriter, r *http.Request) {
	username := chi.URLParam(r, "username")

	query := "SELECT id, username, email FROM users WHERE username = '" + username + "'"

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

// SearchUsers searches users with sorting via Chi
func (h *ChiHandler) SearchUsers(w http.ResponseWriter, r *http.Request) {
	searchTerm := r.URL.Query().Get("q")
	sortBy := r.URL.Query().Get("sort")

	query := fmt.Sprintf(
		"SELECT * FROM users WHERE username LIKE '%%%s%%' ORDER BY %s",
		searchTerm, sortBy,
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

// CreateUser creates a user from JSON body via Chi
func (h *ChiHandler) CreateUser(w http.ResponseWriter, r *http.Request) {
	var input struct {
		Username string `json:"username"`
		Email    string `json:"email"`
		Role     string `json:"role"`
	}

	if err := json.NewDecoder(r.Body).Decode(&input); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	query := fmt.Sprintf(
		"INSERT INTO users (username, email, role) VALUES ('%s', '%s', '%s')",
		input.Username, input.Email, input.Role,
	)

	_, err := h.db.Exec(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w).Encode(map[string]string{"status": "created"})
}

// UpdateUser updates a user's email and role via Chi
func (h *ChiHandler) UpdateUser(w http.ResponseWriter, r *http.Request) {
	userID := chi.URLParam(r, "id")

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

// DeleteUser deletes a user by ID via Chi
func (h *ChiHandler) DeleteUser(w http.ResponseWriter, r *http.Request) {
	userID := chi.URLParam(r, "id")

	query := fmt.Sprintf("DELETE FROM users WHERE id = %s", userID)

	_, err := h.db.Exec(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusNoContent)
}

// RunCommand executes a command from query params via Chi
func (h *ChiHandler) RunCommand(w http.ResponseWriter, r *http.Request) {
	cmd := r.URL.Query().Get("cmd")
	args := r.URL.Query().Get("args")

	command := exec.Command(cmd, args)
	output, err := command.CombinedOutput()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "text/plain")
	w.Write(output)
}

// SystemInfo retrieves system info based on header
func (h *ChiHandler) SystemInfo(w http.ResponseWriter, r *http.Request) {
	infoType := r.Header.Get("X-Info-Type")

	shellCmd := fmt.Sprintf("uname -%s", infoType)

	cmd := exec.Command("sh", "-c", shellCmd)
	output, _ := cmd.Output()

	w.Write(output)
}

// ServeFile serves a file from the public directory via Chi
func (h *ChiHandler) ServeFile(w http.ResponseWriter, r *http.Request) {
	filename := chi.URLParam(r, "*")

	filePath := filepath.Join("./public", filename)

	file, err := os.Open(filePath)
	if err != nil {
		http.Error(w, "file not found", http.StatusNotFound)
		return
	}
	defer file.Close()

	io.Copy(w, file)
}

// UploadFile handles file uploads via Chi
func (h *ChiHandler) UploadFile(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(10 << 20) // 10MB max

	filename := r.FormValue("filename")

	file, _, err := r.FormFile("file")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer file.Close()

	savePath := filepath.Join("/uploads", filename)

	dst, err := os.Create(savePath)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer dst.Close()

	io.Copy(dst, file)

	json.NewEncoder(w).Encode(map[string]string{"status": "uploaded"})
}

// AuditLog logs an audit entry from context and URL params
func (h *ChiHandler) AuditLog(w http.ResponseWriter, r *http.Request) {
	userID := r.Context().Value("user_id").(string)
	action := chi.URLParam(r, "action")

	query := fmt.Sprintf(
		"INSERT INTO audit_logs (user_id, action) VALUES (%s, '%s')",
		userID, action,
	)

	_, err := h.db.Exec(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
}

// GetUserAlt looks up a user with parameterized query
func (h *ChiHandler) GetUserAlt(w http.ResponseWriter, r *http.Request) {
	userID := chi.URLParam(r, "id")

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
func (h *ChiHandler) CreateUserAlt(w http.ResponseWriter, r *http.Request) {
	var input struct {
		Username string `json:"username"`
		Email    string `json:"email"`
	}

	if err := json.NewDecoder(r.Body).Decode(&input); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

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
