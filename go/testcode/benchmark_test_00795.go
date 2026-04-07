package testcode

import (
	"crypto/sha1"
	"fmt"
	"net/http"
)

func BenchmarkTest00795(w http.ResponseWriter, r *http.Request) {
	password := r.FormValue("password")
	username := r.FormValue("username")
	salt := "static-salt-value"
	h := sha1.Sum([]byte(salt + password))
	hashStr := fmt.Sprintf("%x", h)
	_, err := DB.Exec("INSERT INTO users (username, pw_hash) VALUES (?, ?)", username, hashStr)
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusCreated, map[string]string{"status": "ok"})
}
