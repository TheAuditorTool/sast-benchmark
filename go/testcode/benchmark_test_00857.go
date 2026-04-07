package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"
)

func benchmarkTest00857SetCookie(w http.ResponseWriter, name, value string) {
	http.SetCookie(w, &http.Cookie{
		Name:     name,
		Value:    value,
		HttpOnly: true,
		Path:     "/",
	})
}

func BenchmarkTest00857(w http.ResponseWriter, r *http.Request) {
	b := make([]byte, 16)
	rand.Read(b)
	benchmarkTest00857SetCookie(w, "session", hex.EncodeToString(b))
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
