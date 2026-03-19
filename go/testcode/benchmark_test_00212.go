package testcode

import (
	"fmt"
	"net/http"
	"time"
)

func BenchmarkTest00212(w http.ResponseWriter, r *http.Request) {
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

	http.SetCookie(w, &http.Cookie{
		Name:    "user_session",
		Value:   fmt.Sprintf("uid_%d", userID),
		Path:    "/",
		Expires: time.Now().Add(7 * 24 * time.Hour),
	})

	RespondJSON(w, http.StatusOK, map[string]string{
		"message": "logged in",
		"user_id": fmt.Sprintf("%d", userID),
	})
}
