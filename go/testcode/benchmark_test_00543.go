package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00543(w http.ResponseWriter, r *http.Request) {
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

	raw := make([]byte, 32)
	if _, err := rand.Read(raw); err != nil {
		http.Error(w, "rng error", http.StatusInternalServerError)
		return
	}
	sessionID := hex.EncodeToString(raw)

	_, err = DB.Exec(
		"INSERT INTO sessions (token, user_id) VALUES (?, ?)",
		sessionID, userID,
	)
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}

	http.SetCookie(w, &http.Cookie{
		Name:     "__Secure-session",
		Value:    sessionID,
		Secure:   true,
		HttpOnly: true,
		SameSite: http.SameSiteStrictMode,
		Path:     "/",
		MaxAge:   3600,
	})

	RespondJSON(w, http.StatusOK, map[string]string{"user_id": userID})
}
