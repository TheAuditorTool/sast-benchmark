package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"
)

func benchmarkTest00870SetSecureCookie(w http.ResponseWriter, name, value string) {
	http.SetCookie(w, &http.Cookie{
		Name:     name,
		Value:    value,
		Secure:   true,
		HttpOnly: true,
		SameSite: http.SameSiteStrictMode,
		Path:     "/",
	})
}

func BenchmarkTest00870(w http.ResponseWriter, r *http.Request) {
	b := make([]byte, 32)
	rand.Read(b)
	benchmarkTest00870SetSecureCookie(w, "session", hex.EncodeToString(b))
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
