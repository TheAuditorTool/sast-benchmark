package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"io"
	"net/http"

	"golang.org/x/crypto/chacha20poly1305"
)

func BenchmarkTest00200(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Plaintext string `json:"plaintext"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	key := make([]byte, chacha20poly1305.KeySize)
	if _, err := io.ReadFull(rand.Reader, key); err != nil {
		http.Error(w, "key generation failed", http.StatusInternalServerError)
		return
	}

	aead, err := chacha20poly1305.New(key)
	if err != nil {
		http.Error(w, "cipher init failed", http.StatusInternalServerError)
		return
	}

	nonce := make([]byte, aead.NonceSize())
	if _, err := io.ReadFull(rand.Reader, nonce); err != nil {
		http.Error(w, "nonce generation failed", http.StatusInternalServerError)
		return
	}

	ciphertext := aead.Seal(nonce, nonce, []byte(req.Plaintext), nil)

	RespondJSON(w, http.StatusOK, map[string]string{
		"ciphertext": hex.EncodeToString(ciphertext),
		"key":        hex.EncodeToString(key),
		"algorithm":  "chacha20-poly1305",
	})
}
