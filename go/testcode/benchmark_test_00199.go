package testcode

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/hex"
	"io"
	"net/http"
)

func BenchmarkTest00199(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Plaintext string `json:"plaintext"`
		Key       string `json:"key"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	keyBytes := []byte(req.Key)
	if len(keyBytes) < 16 {
		keyBytes = append(keyBytes, make([]byte, 16-len(keyBytes))...)
	}

	block, err := aes.NewCipher(keyBytes[:16])
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
		"algorithm":  "aes-gcm",
	})
}
