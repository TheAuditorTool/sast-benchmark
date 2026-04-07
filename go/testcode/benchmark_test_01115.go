package testcode

import (
	"net/http"
)

func BenchmarkTest01115(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	password := r.FormValue("password")

	var storedPassword string
	row := DB.QueryRow("SELECT password FROM users WHERE username = ?", username)
	if err := row.Scan(&storedPassword); err != nil {
		http.Error(w, "invalid credentials", http.StatusUnauthorized)
		return
	}

	if password == storedPassword {
		RespondJSON(w, http.StatusOK, map[string]string{"status": "authenticated"})
		return
	}

	http.Error(w, "invalid credentials", http.StatusUnauthorized)
}
