package testcode

import (
	"crypto/sha1"
	"fmt"
	"net/http"
)

func BenchmarkTest00784(w http.ResponseWriter, r *http.Request) {
	password := r.FormValue("password")
	username := r.FormValue("username")
	hash := sha1.Sum([]byte(password))
	hashStr := fmt.Sprintf("%x", hash)
	_, err := DB.Exec("INSERT INTO users (username, password_hash) VALUES (?, ?)", username, hashStr)
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusCreated, map[string]string{"status": "created"})
}
