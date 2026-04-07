package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00855(w http.ResponseWriter, r *http.Request) {
	b := make([]byte, 32)
	rand.Read(b)
	http.SetCookie(w, &http.Cookie{
		Name:     "access_token",
		Value:    hex.EncodeToString(b),
		MaxAge:   86400 * 365,
		HttpOnly: true,
		Path:     "/",
	})
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
