package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00214(w http.ResponseWriter, r *http.Request) {
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

	tokenBytes := make([]byte, 32)
	rand.Read(tokenBytes)
	token := hex.EncodeToString(tokenBytes)

	http.SetCookie(w, &http.Cookie{
		Name:   "session",
		Value:  token,
		MaxAge: 0,
		Path:   "/",
	})

	RespondJSON(w, http.StatusOK, map[string]string{"message": "logged in"})
}
