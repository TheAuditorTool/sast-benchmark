package testcode

import (
	"fmt"
	"math/rand"
	"net/http"
	"time"
)

func BenchmarkTest00155(w http.ResponseWriter, r *http.Request) {
	var req struct {
		UserID string `json:"user_id"`
		Scope  string `json:"scope"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	rand.Seed(time.Now().UnixNano())
	apiKey := fmt.Sprintf("ak_%016x%016x", rand.Int63(), rand.Int63())

	_, err := DB.Exec("INSERT INTO api_keys (user_id, key_value, scope, created_at) VALUES (?, ?, ?, ?)",
		req.UserID, apiKey, req.Scope, time.Now().Unix())
	if err != nil {
		http.Error(w, "key generation failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"api_key": apiKey,
		"scope":   req.Scope,
	})
}
