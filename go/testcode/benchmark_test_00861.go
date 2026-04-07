package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00861(w http.ResponseWriter, r *http.Request) {
	b := make([]byte, 32)
	rand.Read(b)
	http.SetCookie(w, &http.Cookie{
		Name:     "session",
		Value:    hex.EncodeToString(b),
		Secure:   true,
		HttpOnly: true,
		SameSite: http.SameSiteLaxMode,
		Path:     "/",
	})
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
