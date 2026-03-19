package testcode

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/hex"
	"io"
	"net/http"
)

func BenchmarkTest00205(w http.ResponseWriter, r *http.Request) {
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

	sealed := aesGCM.Seal(nonce, nonce, []byte(req.Plaintext), nil)

	_, err = DB.Exec("INSERT INTO secrets (ciphertext, created_at) VALUES (?, datetime('now'))",
		hex.EncodeToString(sealed))
	if err != nil {
		http.Error(w, "storage failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"ciphertext": hex.EncodeToString(sealed),
		"algorithm":  "aes-gcm-sealed",
	})
}
