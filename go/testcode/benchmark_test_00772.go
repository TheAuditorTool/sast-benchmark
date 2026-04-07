package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00772(w http.ResponseWriter, r *http.Request) {
	b := make([]byte, 16)
	if _, err := rand.Read(b); err != nil {
		http.Error(w, "random error", http.StatusInternalServerError)
		return
	}
	sessionID := hex.EncodeToString(b)
	http.SetCookie(w, &http.Cookie{
		Name:     "session_id",
		Value:    sessionID,
		HttpOnly: true,
		Secure:   true,
		SameSite: http.SameSiteStrictMode,
	})
	RespondJSON(w, http.StatusOK, map[string]string{"status": "session created"})
}
