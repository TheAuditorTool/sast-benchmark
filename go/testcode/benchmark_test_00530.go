package testcode

import (
	"net/http"
	"net/mail"
)

func BenchmarkTest00530(w http.ResponseWriter, r *http.Request) {
	email := r.URL.Query().Get("email")
	if email == "" {
		http.Error(w, "email required", http.StatusBadRequest)
		return
	}

	addr, err := mail.ParseAddress(email)
	if err != nil {
		http.Error(w, "invalid email format", http.StatusBadRequest)
		return
	}

	var userID string
	err = DB.QueryRow("SELECT id FROM users WHERE email = ?", addr.Address).Scan(&userID)
	if err != nil {
		http.Error(w, "user not found", http.StatusNotFound)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"user_id": userID, "email": addr.Address})
}
