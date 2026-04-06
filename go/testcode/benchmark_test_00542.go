package testcode

import (
	"net/http"
)

func BenchmarkTest00542(w http.ResponseWriter, r *http.Request) {
	var body struct {
		Username string `json:"username"`
		Password string `json:"password"`
	}
	if err := ParseJSONBody(r, &body); err != nil {
		http.Error(w, "invalid request", http.StatusBadRequest)
		return
	}

	var token string
	err := DB.QueryRow(
		"SELECT session_token FROM users WHERE username = ? AND password_hash = ?",
		body.Username, body.Password,
	).Scan(&token)
	if err != nil {
		http.Error(w, "invalid credentials", http.StatusUnauthorized)
		return
	}

	http.SetCookie(w, &http.Cookie{
		Name:     "session_id",
		Value:    token,
		Path:     "/",
		HttpOnly: true,
	})

	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
