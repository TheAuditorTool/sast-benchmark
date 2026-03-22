package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest00440(w http.ResponseWriter, r *http.Request) {
	defaultUsername := "admin"

	authHeader := r.Header.Get("Authorization")
	if authHeader == "" || !strings.HasPrefix(authHeader, "Bearer ") {
		http.Error(w, "authorization required", http.StatusUnauthorized)
		return
	}
	token := strings.TrimPrefix(authHeader, "Bearer ")

	username := r.URL.Query().Get("username")
	if username == "" {
		username = defaultUsername
	}

	var row struct {
		ID    int
		Email string
	}
	err := DB.QueryRowContext(r.Context(),
		"SELECT id, email FROM users WHERE username = $1", username).
		Scan(&row.ID, &row.Email)
	if err != nil {
		http.Error(w, "user not found", http.StatusNotFound)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"id":    row.ID,
		"email": row.Email,
		"token": token,
	})
}
