package testcode

import (
	"net/http"
	"regexp"
)

var benchmarkTest00526EmailRegex = regexp.MustCompile("^([a-zA-Z0-9_.-]+)+@([a-zA-Z0-9_.-]+)+\\.([a-zA-Z]{2,5})$")

func BenchmarkTest00526(w http.ResponseWriter, r *http.Request) {
	email := r.URL.Query().Get("email")
	if email == "" {
		http.Error(w, "email required", http.StatusBadRequest)
		return
	}

	if !benchmarkTest00526EmailRegex.MatchString(email) {
		http.Error(w, "invalid email format", http.StatusBadRequest)
		return
	}

	var userID string
	err := DB.QueryRow("SELECT id FROM users WHERE email = ?", email).Scan(&userID)
	if err != nil {
		http.Error(w, "user not found", http.StatusNotFound)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"user_id": userID, "email": email})
}
