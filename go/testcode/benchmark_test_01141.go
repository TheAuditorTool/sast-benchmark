package testcode

import (
	"net/http"

	"golang.org/x/crypto/bcrypt"
)

func BenchmarkTest01141(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	password := r.FormValue("password")

	var hashedPassword string
	var active bool
	row := DB.QueryRow(
		"SELECT password, active FROM users WHERE username = ?",
		username,
	)
	if err := row.Scan(&hashedPassword, &active); err != nil {
		http.Error(w, "invalid credentials", http.StatusUnauthorized)
		return
	}

	if !active {
		http.Error(w, "account disabled", http.StatusForbidden)
		return
	}

	if err := bcrypt.CompareHashAndPassword([]byte(hashedPassword), []byte(password)); err != nil {
		http.Error(w, "invalid credentials", http.StatusUnauthorized)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "authenticated"})
}
