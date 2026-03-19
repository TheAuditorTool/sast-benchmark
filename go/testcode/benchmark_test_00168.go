package testcode

import (
	"crypto/rand"
	"encoding/base64"
	"net/http"
	"time"
)

func BenchmarkTest00168(w http.ResponseWriter, r *http.Request) {
	var req struct {
		UserID string `json:"user_id"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	tokenBytes := make([]byte, 24)
	if _, err := rand.Read(tokenBytes); err != nil {
		http.Error(w, "token generation failed", http.StatusInternalServerError)
		return
	}

	token := base64.URLEncoding.EncodeToString(tokenBytes)

	_, err := DB.Exec("INSERT INTO refresh_tokens (user_id, token, expires_at) VALUES (?, ?, ?)",
		req.UserID, token, time.Now().Add(7*24*time.Hour).Unix())
	if err != nil {
		http.Error(w, "token storage failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"refresh_token": token,
	})
}
