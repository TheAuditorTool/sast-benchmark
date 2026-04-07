package testcode

import (
	"net/http"

	"golang.org/x/crypto/bcrypt"
)

func BenchmarkTest01129(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	password := r.FormValue("password")

	var hashedPassword string
	row := DB.QueryRow("SELECT password FROM users WHERE username = ?", username)
	if err := row.Scan(&hashedPassword); err != nil {
		http.Error(w, "invalid credentials", http.StatusUnauthorized)
		return
	}

	if err := bcrypt.CompareHashAndPassword([]byte(hashedPassword), []byte(password)); err != nil {
		http.Error(w, "invalid credentials", http.StatusUnauthorized)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "authenticated"})
}
