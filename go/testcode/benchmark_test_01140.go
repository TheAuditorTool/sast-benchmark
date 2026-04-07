package testcode

import (
	"net/http"

	"golang.org/x/crypto/bcrypt"
)

func BenchmarkTest01140(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	password := r.FormValue("password")

	var userID int
	var hashedPassword string
	var failedAttempts int
	row := DB.QueryRow(
		"SELECT id, password, failed_attempts FROM users WHERE username = ?",
		username,
	)
	if err := row.Scan(&userID, &hashedPassword, &failedAttempts); err != nil {
		http.Error(w, "invalid credentials", http.StatusUnauthorized)
		return
	}

	if failedAttempts >= 5 {
		http.Error(w, "account locked", http.StatusForbidden)
		return
	}

	if err := bcrypt.CompareHashAndPassword([]byte(hashedPassword), []byte(password)); err != nil {
		DB.Exec("UPDATE users SET failed_attempts = failed_attempts + 1 WHERE id = ?", userID)
		http.Error(w, "invalid credentials", http.StatusUnauthorized)
		return
	}

	DB.Exec("UPDATE users SET failed_attempts = 0 WHERE id = ?", userID)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "authenticated"})
}
