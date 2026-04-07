package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"io"
	"net/http"
	"os"

	"golang.org/x/crypto/chacha20poly1305"
)

func BenchmarkTest00838(w http.ResponseWriter, r *http.Request) {
	keyHex := os.Getenv("CHACHA_KEY")
	key, err := hex.DecodeString(keyHex)
	if err != nil || len(key) != chacha20poly1305.KeySize {
		http.Error(w, "key error", http.StatusInternalServerError)
		return
	}
	aead, _ := chacha20poly1305.New(key)
	nonce := make([]byte, aead.NonceSize())
	io.ReadFull(rand.Reader, nonce)
	ct := aead.Seal(nonce, nonce, []byte(r.FormValue("data")), nil)
	RespondJSON(w, http.StatusOK, map[string]string{"ct": hex.EncodeToString(ct)})
}
