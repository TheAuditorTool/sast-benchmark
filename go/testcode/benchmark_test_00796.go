package testcode

import (
	"crypto/sha256"
	"fmt"
	"net/http"
)

func BenchmarkTest00796(w http.ResponseWriter, r *http.Request) {
	password := r.FormValue("password")
	username := r.FormValue("username")
	full := sha256.Sum256([]byte(password))
	truncated := fmt.Sprintf("%x", full[:4])
	_, err := DB.Exec("INSERT INTO users (username, pw_hash) VALUES (?, ?)", username, truncated)
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusCreated, map[string]string{"status": "ok"})
}
