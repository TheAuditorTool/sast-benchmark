package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00851(w http.ResponseWriter, r *http.Request) {
	b := make([]byte, 16)
	rand.Read(b)
	http.SetCookie(w, &http.Cookie{
		Name:  "sess",
		Value: hex.EncodeToString(b),
	})
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
