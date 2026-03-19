package testcode

import (
	"crypto/sha1"
	"fmt"
	"net/http"
	"time"
)

func BenchmarkTest00173(w http.ResponseWriter, r *http.Request) {
	var req struct {
		UserID  string `json:"user_id"`
		Payload string `json:"payload"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	data := fmt.Sprintf("%s:%s:%d", req.UserID, req.Payload, time.Now().Unix())
	hash := sha1.Sum([]byte(data))
	integrity := fmt.Sprintf("%x", hash)

	_, err := DB.Exec("INSERT INTO auth_log (user_id, payload_hash, verified) VALUES (?, ?, 1)",
		req.UserID, integrity)
	if err != nil {
		http.Error(w, "logging failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"integrity_hash": integrity,
		"status":         "verified",
	})
}
