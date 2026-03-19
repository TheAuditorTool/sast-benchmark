package testcode

import (
	"net/http"
)

func BenchmarkTest00236(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Username string `json:"username"`
		Password string `json:"password"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	var userID int
	err := DB.QueryRow("SELECT id FROM users WHERE username = ? AND password_hash = ?",
		req.Username, req.Password).Scan(&userID)
	if err != nil {
		http.Error(w, "invalid credentials", http.StatusUnauthorized)
		return
	}

	http.Redirect(w, r, "/login", http.StatusFound)
}
