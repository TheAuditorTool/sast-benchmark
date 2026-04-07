package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00852(w http.ResponseWriter, r *http.Request) {
	b := make([]byte, 16)
	rand.Read(b)
	w.Header().Set("Set-Cookie", "sess="+hex.EncodeToString(b)+"; Path=/")
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
