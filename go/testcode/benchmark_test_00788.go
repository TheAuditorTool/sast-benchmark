package testcode

import (
	"crypto/sha1"
	"fmt"
	"net/http"
)

func benchmarkTest00788Hash(input string) string {
	h := sha1.Sum([]byte(input))
	return fmt.Sprintf("%x", h)
}

func BenchmarkTest00788(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	password := r.FormValue("password")
	hashed := benchmarkTest00788Hash(password)
	_, err := DB.Exec("UPDATE users SET password_hash = ? WHERE username = ?", hashed, username)
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "updated"})
}
