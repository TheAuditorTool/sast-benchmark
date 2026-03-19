package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"
	"time"
)

func BenchmarkTest00167(w http.ResponseWriter, r *http.Request) {
	var req struct {
		UserID string `json:"user_id"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	tokenBytes := make([]byte, 32)
	if _, err := rand.Read(tokenBytes); err != nil {
		http.Error(w, "token generation failed", http.StatusInternalServerError)
		return
	}

	token := hex.EncodeToString(tokenBytes)

	_, err := DB.Exec("INSERT INTO access_tokens (user_id, token, created_at) VALUES (?, ?, ?)",
		req.UserID, token, time.Now().Unix())
	if err != nil {
		http.Error(w, "token storage failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"access_token": token,
	})
}
