package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"
)

type benchmarkTest00848CookieOpts struct {
	Name     string
	Secure   bool
	HttpOnly bool
}

func BenchmarkTest00848(w http.ResponseWriter, r *http.Request) {
	opts := benchmarkTest00848CookieOpts{Name: "session", Secure: false, HttpOnly: true}
	token := make([]byte, 16)
	rand.Read(token)
	http.SetCookie(w, &http.Cookie{
		Name:     opts.Name,
		Value:    hex.EncodeToString(token),
		Secure:   opts.Secure,
		HttpOnly: opts.HttpOnly,
	})
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
