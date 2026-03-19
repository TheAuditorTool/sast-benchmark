package testcode

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/hex"
	"io"
	"net/http"
)

func BenchmarkTest00197(w http.ResponseWriter, r *http.Request) {
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

	plainBytes := []byte(req.Plaintext)
	padLen := aes.BlockSize - (len(plainBytes) % aes.BlockSize)
	padded := append(plainBytes, make([]byte, padLen)...)
	for i := len(plainBytes); i < len(padded); i++ {
		padded[i] = byte(padLen)
	}

	iv := make([]byte, aes.BlockSize)
	if _, err := io.ReadFull(rand.Reader, iv); err != nil {
		http.Error(w, "iv generation failed", http.StatusInternalServerError)
		return
	}

	ciphertext := make([]byte, len(padded))
	mode := cipher.NewCBCEncrypter(block, iv)
	mode.CryptBlocks(ciphertext, padded)

	result := append(iv, ciphertext...)

	RespondJSON(w, http.StatusOK, map[string]string{
		"ciphertext": hex.EncodeToString(result),
		"algorithm":  "aes-cbc",
	})
}
