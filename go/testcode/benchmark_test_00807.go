package testcode

import (
	"net/http"

	"golang.org/x/crypto/bcrypt"
)

func BenchmarkTest00807(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	password := r.FormValue("password")
	var storedHash string
	err := DB.QueryRow("SELECT pw_hash FROM users WHERE username = ?", username).Scan(&storedHash)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	if bcrypt.CompareHashAndPassword([]byte(storedHash), []byte(password)) != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
