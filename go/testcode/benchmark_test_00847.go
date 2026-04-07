package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00847(w http.ResponseWriter, r *http.Request) {
	token := make([]byte, 32)
	rand.Read(token)
	http.SetCookie(w, &http.Cookie{
		Name:     "remember_me",
		Value:    hex.EncodeToString(token),
		Domain:   ".example.com",
		HttpOnly: true,
		MaxAge:   86400 * 30,
	})
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
