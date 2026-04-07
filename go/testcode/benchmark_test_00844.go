package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00844(w http.ResponseWriter, r *http.Request) {
	token := make([]byte, 16)
	rand.Read(token)
	http.SetCookie(w, &http.Cookie{
		Name:     "auth",
		Value:    hex.EncodeToString(token),
		Secure:   true,
		HttpOnly: false,
	})
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
