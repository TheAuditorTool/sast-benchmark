package testcode

import (
	"crypto/aes"
	"crypto/cipher"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00206(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Ciphertext string `json:"ciphertext"`
		Key        string `json:"key"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	keyBytes, err := hex.DecodeString(req.Key)
	if err != nil || len(keyBytes) != 32 {
		http.Error(w, "invalid key", http.StatusBadRequest)
		return
	}

	ciphertextBytes, err := hex.DecodeString(req.Ciphertext)
	if err != nil {
		http.Error(w, "invalid ciphertext", http.StatusBadRequest)
		return
	}

	block, err := aes.NewCipher(keyBytes)
	if err != nil {
		http.Error(w, "cipher init failed", http.StatusInternalServerError)
		return
	}

	aesGCM, err := cipher.NewGCM(block)
	if err != nil {
		http.Error(w, "gcm init failed", http.StatusInternalServerError)
		return
	}

	nonceSize := aesGCM.NonceSize()
	if len(ciphertextBytes) < nonceSize {
		http.Error(w, "ciphertext too short", http.StatusBadRequest)
		return
	}

	nonce := ciphertextBytes[:nonceSize]
	ct := ciphertextBytes[nonceSize:]

	plaintext, err := aesGCM.Open(nil, nonce, ct, nil)
	if err != nil {
		http.Error(w, "decryption failed - authentication error", http.StatusForbidden)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"plaintext": string(plaintext),
		"algorithm": "aes-gcm-open",
	})
}
