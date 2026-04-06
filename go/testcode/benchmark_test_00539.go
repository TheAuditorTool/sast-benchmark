package testcode

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/hex"
	"io"
	"net/http"
)

var benchmarkTest00539Key = []byte("f3a1b2c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2")

func BenchmarkTest00539(w http.ResponseWriter, r *http.Request) {
	var body struct {
		Plaintext string `json:"plaintext"`
	}
	if err := ParseJSONBody(r, &body); err != nil {
		http.Error(w, "invalid request", http.StatusBadRequest)
		return
	}

	key, err := hex.DecodeString(string(benchmarkTest00539Key))
	if err != nil || len(key) != 32 {
		http.Error(w, "key error", http.StatusInternalServerError)
		return
	}

	block, err := aes.NewCipher(key)
	if err != nil {
		http.Error(w, "cipher error", http.StatusInternalServerError)
		return
	}

	gcm, err := cipher.NewGCM(block)
	if err != nil {
		http.Error(w, "gcm error", http.StatusInternalServerError)
		return
	}

	nonce := make([]byte, gcm.NonceSize())
	if _, err := io.ReadFull(rand.Reader, nonce); err != nil {
		http.Error(w, "rng error", http.StatusInternalServerError)
		return
	}

	sealed := gcm.Seal(nonce, nonce, []byte(body.Plaintext), nil)
	RespondJSON(w, http.StatusOK, map[string]string{
		"ciphertext": hex.EncodeToString(sealed),
	})
}
