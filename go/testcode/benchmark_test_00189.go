package testcode

import (
	"crypto/sha256"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00189(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Action    string `json:"action"`
		CSRFToken string `json:"csrf_token"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	sessionID := r.Header.Get("X-Session-ID")
	if sessionID == "" {
		http.Error(w, "no session", http.StatusUnauthorized)
		return
	}

	var storedSecret string
	err := DB.QueryRow("SELECT csrf_secret FROM sessions WHERE id = ?", sessionID).Scan(&storedSecret)
	if err != nil {
		http.Error(w, "session not found", http.StatusUnauthorized)
		return
	}

	hash := sha256.Sum256([]byte(storedSecret + sessionID))
	expected := hex.EncodeToString(hash[:])

	if expected != req.CSRFToken {
		http.Error(w, "CSRF verification failed", http.StatusForbidden)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"status": "csrf verified",
		"action": req.Action,
	})
}
