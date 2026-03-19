package testcode

import (
	"crypto/md5"
	"fmt"
	"net/http"
)

func BenchmarkTest00172(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Username string `json:"username"`
		Password string `json:"password"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	h := md5.New()
	h.Write([]byte(req.Password))
	hashBytes := h.Sum(nil)
	hashStr := fmt.Sprintf("%x", hashBytes)

	var storedHash string
	err := DB.QueryRow("SELECT password_hash FROM users WHERE username = ?", req.Username).Scan(&storedHash)
	if err != nil {
		http.Error(w, "user not found", http.StatusUnauthorized)
		return
	}

	if hashStr != storedHash {
		http.Error(w, "invalid credentials", http.StatusUnauthorized)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"message": "login successful",
	})
}
