package testcode

import (
	"net/http"

	"github.com/google/uuid"
)

func BenchmarkTest00782(w http.ResponseWriter, r *http.Request) {
	resetToken := uuid.New().String()
	email := r.URL.Query().Get("email")
	_, err := DB.Exec("INSERT INTO password_resets (email, token) VALUES (?, ?)", email, resetToken)
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"message": "reset email sent"})
}
