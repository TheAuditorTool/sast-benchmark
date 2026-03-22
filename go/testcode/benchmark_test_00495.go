package testcode

import (
	"log"
	"net/http"
)

func BenchmarkTest00495(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	password := r.FormValue("password")

	var storedHash string
	err := DB.QueryRow("SELECT password_hash FROM users WHERE username = ?", username).Scan(&storedHash)
	if err != nil {
		log.Printf("user login: %s from IP: %s - user not found", username, r.RemoteAddr)
		http.Error(w, "invalid credentials", http.StatusUnauthorized)
		return
	}

	if storedHash != password {
		log.Printf("user login: %s from IP: %s - wrong password", username, r.RemoteAddr)
		http.Error(w, "invalid credentials", http.StatusUnauthorized)
		return
	}

	log.Printf("user login: %s from IP: %s - success", username, r.RemoteAddr)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "logged_in"})
}
