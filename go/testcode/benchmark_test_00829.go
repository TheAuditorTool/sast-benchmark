package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"io"
	"net/http"

	"golang.org/x/crypto/chacha20poly1305"
)

func BenchmarkTest00829(w http.ResponseWriter, r *http.Request) {
	key := make([]byte, chacha20poly1305.KeySize)
	io.ReadFull(rand.Reader, key)
	aead, _ := chacha20poly1305.New(key)
	nonce := make([]byte, aead.NonceSize())
	io.ReadFull(rand.Reader, nonce)
	ct := aead.Seal(nonce, nonce, []byte(r.FormValue("data")), nil)
	RespondJSON(w, http.StatusOK, map[string]string{"ct": hex.EncodeToString(ct)})
}
