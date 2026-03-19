package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"
	"time"
)

func BenchmarkTest00219(w http.ResponseWriter, r *http.Request) {
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

	sessionIDBytes := make([]byte, 32)
	rand.Read(sessionIDBytes)
	sessionID := hex.EncodeToString(sessionIDBytes)

	_, err = DB.Exec("INSERT INTO sessions (session_id, user_id, created_at, expires_at) VALUES (?, ?, ?, ?)",
		sessionID, userID, time.Now().Unix(), time.Now().Add(4*time.Hour).Unix())
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}

	http.SetCookie(w, &http.Cookie{
		Name:     "sid",
		Value:    sessionID,
		Secure:   true,
		HttpOnly: true,
		SameSite: http.SameSiteStrictMode,
		Path:     "/",
		MaxAge:   14400,
	})

	RespondJSON(w, http.StatusOK, map[string]string{"message": "logged in"})
}
