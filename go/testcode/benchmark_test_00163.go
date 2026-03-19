package testcode

import (
	"net/http"
	"time"

	"github.com/google/uuid"
)

func BenchmarkTest00163(w http.ResponseWriter, r *http.Request) {
	var req struct {
		UserID string `json:"user_id"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	token := uuid.New().String()

	_, err := DB.Exec("INSERT INTO auth_tokens (user_id, token, expires_at) VALUES (?, ?, ?)",
		req.UserID, token, time.Now().Add(24*time.Hour).Unix())
	if err != nil {
		http.Error(w, "token creation failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"token": token,
	})
}
