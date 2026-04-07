package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00858(w http.ResponseWriter, r *http.Request) {
	b := make([]byte, 32)
	rand.Read(b)
	http.SetCookie(w, &http.Cookie{
		Name:     "session",
		Value:    hex.EncodeToString(b),
		Secure:   true,
		HttpOnly: true,
		SameSite: http.SameSiteStrictMode,
		Path:     "/",
	})
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
