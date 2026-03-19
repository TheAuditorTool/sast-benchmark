package testcode

import (
	"encoding/hex"
	"fmt"
	"math/rand"
	"net/http"
	"time"
)

func BenchmarkTest00151(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Username string `json:"username"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	rand.Seed(time.Now().UnixNano())
	tokenBytes := make([]byte, 16)
	for i := range tokenBytes {
		tokenBytes[i] = byte(rand.Intn(256))
	}
	sessionToken := hex.EncodeToString(tokenBytes)

	_, err := DB.Exec("INSERT INTO sessions (username, token, created_at) VALUES (?, ?, ?)",
		req.Username, sessionToken, time.Now().Unix())
	if err != nil {
		http.Error(w, "session creation failed", http.StatusInternalServerError)
		return
	}

	http.SetCookie(w, &http.Cookie{
		Name:     "session",
		Value:    sessionToken,
		Path:     "/",
		HttpOnly: true,
		Expires:  time.Now().Add(24 * time.Hour),
	})

	RespondJSON(w, http.StatusOK, map[string]string{
		"message": fmt.Sprintf("session created for %s", req.Username),
	})
}
