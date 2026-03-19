package testcode

import (
	"crypto/rand"
	"encoding/base64"
	"net/http"
	"time"
)

func BenchmarkTest00161(w http.ResponseWriter, r *http.Request) {
	var req struct {
		KeyName string `json:"key_name"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	buf := make([]byte, 32)
	if _, err := rand.Read(buf); err != nil {
		http.Error(w, "key generation failed", http.StatusInternalServerError)
		return
	}

	keyValue := base64.StdEncoding.EncodeToString(buf)

	_, err := DB.Exec("INSERT INTO encryption_keys (name, material, created_at) VALUES (?, ?, ?)",
		req.KeyName, keyValue, time.Now().Unix())
	if err != nil {
		http.Error(w, "storage failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"key_name": req.KeyName,
		"key":      keyValue,
	})
}
