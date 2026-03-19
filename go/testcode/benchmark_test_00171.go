package testcode

import (
	"crypto/md5"
	"fmt"
	"net/http"
)

func BenchmarkTest00171(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Username string `json:"username"`
		Password string `json:"password"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	hash := md5.Sum([]byte(req.Password))
	hashStr := fmt.Sprintf("%x", hash)

	_, err := DB.Exec("INSERT INTO users (username, password_hash) VALUES (?, ?)",
		req.Username, hashStr)
	if err != nil {
		http.Error(w, "registration failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"message": "user registered",
	})
}
