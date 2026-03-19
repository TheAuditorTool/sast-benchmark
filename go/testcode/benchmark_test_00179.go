package testcode

import (
	"crypto/sha1"
	"fmt"
	"net/http"
	"time"
)

func BenchmarkTest00179(w http.ResponseWriter, r *http.Request) {
	var req struct {
		UserID string `json:"user_id"`
		Scope  string `json:"scope"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	h := sha1.New()
	h.Write([]byte(req.UserID))
	h.Write([]byte(req.Scope))
	h.Write([]byte(fmt.Sprintf("%d", time.Now().UnixNano())))
	apiKey := fmt.Sprintf("key_%x", h.Sum(nil))

	_, err := DB.Exec("INSERT INTO api_keys (user_id, key_value, scope) VALUES (?, ?, ?)",
		req.UserID, apiKey, req.Scope)
	if err != nil {
		http.Error(w, "key derivation failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"api_key": apiKey,
		"scope":   req.Scope,
	})
}
