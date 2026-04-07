package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00867(w http.ResponseWriter, r *http.Request) {
	b := make([]byte, 32)
	rand.Read(b)
	http.SetCookie(w, &http.Cookie{
		Name:     "api_session",
		Value:    hex.EncodeToString(b),
		Secure:   true,
		HttpOnly: true,
		SameSite: http.SameSiteStrictMode,
		Path:     "/api/",
	})
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
