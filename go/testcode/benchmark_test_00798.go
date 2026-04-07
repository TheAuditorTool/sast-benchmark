package testcode

import (
	"net/http"

	"golang.org/x/crypto/bcrypt"
)

func BenchmarkTest00798(w http.ResponseWriter, r *http.Request) {
	password := r.FormValue("password")
	username := r.FormValue("username")
	hash, err := bcrypt.GenerateFromPassword([]byte(password), bcrypt.DefaultCost)
	if err != nil {
		http.Error(w, "hash error", http.StatusInternalServerError)
		return
	}
	_, err = DB.Exec("INSERT INTO users (username, pw_hash) VALUES (?, ?)", username, string(hash))
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusCreated, map[string]string{"status": "created"})
}
