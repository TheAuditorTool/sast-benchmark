package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00843(w http.ResponseWriter, r *http.Request) {
	token := make([]byte, 16)
	rand.Read(token)
	http.SetCookie(w, &http.Cookie{
		Name:     "session",
		Value:    hex.EncodeToString(token),
		HttpOnly: true,
	})
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
