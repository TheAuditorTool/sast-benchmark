package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"

	"github.com/gorilla/securecookie"
)

var benchmarkTest00869SC = securecookie.New([]byte("hash-key-32-byte-0123456789abcdef"), []byte("enc-key-32-bytes-0123456789abcde"))

func BenchmarkTest00869(w http.ResponseWriter, r *http.Request) {
	b := make([]byte, 16)
	rand.Read(b)
	encoded, err := benchmarkTest00869SC.Encode("session", hex.EncodeToString(b))
	if err != nil {
		http.Error(w, "encode error", http.StatusInternalServerError)
		return
	}
	http.SetCookie(w, &http.Cookie{
		Name:     "session",
		Value:    encoded,
		Secure:   true,
		HttpOnly: true,
		SameSite: http.SameSiteStrictMode,
		Path:     "/",
	})
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
