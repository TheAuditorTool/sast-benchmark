package testcode

import (
	"fmt"
	"net/http"
	"time"
)

func BenchmarkTest00541(w http.ResponseWriter, r *http.Request) {
	var body struct {
		Username string `json:"username"`
		Password string `json:"password"`
	}
	if err := ParseJSONBody(r, &body); err != nil {
		http.Error(w, "invalid request", http.StatusBadRequest)
		return
	}

	var userID string
	err := DB.QueryRow(
		"SELECT id FROM users WHERE username = ? AND password_hash = ?",
		body.Username, body.Password,
	).Scan(&userID)
	if err != nil {
		http.Error(w, "invalid credentials", http.StatusUnauthorized)
		return
	}

	sessionID := fmt.Sprintf("%d", time.Now().UnixNano())

	http.SetCookie(w, &http.Cookie{
		Name:     "session",
		Value:    sessionID,
		HttpOnly: true,
		SameSite: http.SameSiteNoneMode,
		Secure:   false,
		Path:     "/",
	})

	RespondJSON(w, http.StatusOK, map[string]string{"user_id": userID})
}
