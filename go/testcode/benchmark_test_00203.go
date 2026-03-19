package testcode

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/hex"
	"io"
	"net/http"
)

func BenchmarkTest00203(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Plaintext string `json:"plaintext"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	key := make([]byte, 32)
	if _, err := io.ReadFull(rand.Reader, key); err != nil {
		http.Error(w, "key generation failed", http.StatusInternalServerError)
		return
	}

	block, err := aes.NewCipher(key)
	if err != nil {
		http.Error(w, "cipher init failed", http.StatusInternalServerError)
		return
	}

	aesGCM, err := cipher.NewGCM(block)
	if err != nil {
		http.Error(w, "gcm init failed", http.StatusInternalServerError)
		return
	}

	nonce := make([]byte, aesGCM.NonceSize())
	if _, err := io.ReadFull(rand.Reader, nonce); err != nil {
		http.Error(w, "nonce generation failed", http.StatusInternalServerError)
		return
	}

	ciphertext := aesGCM.Seal(nonce, nonce, []byte(req.Plaintext), nil)

	RespondJSON(w, http.StatusOK, map[string]string{
		"ciphertext": hex.EncodeToString(ciphertext),
		"algorithm":  "aes-256-gcm",
		"key_size":   "256",
	})
}
