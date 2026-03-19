package testcode

import (
	"crypto/aes"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00195(w http.ResponseWriter, r *http.Request) {
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
	padded := make([]byte, ((len(plainBytes)/aes.BlockSize)+1)*aes.BlockSize)
	copy(padded, plainBytes)

	ciphertext := make([]byte, len(padded))
	for i := 0; i < len(padded); i += aes.BlockSize {
		block.Encrypt(ciphertext[i:i+aes.BlockSize], padded[i:i+aes.BlockSize])
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"ciphertext": hex.EncodeToString(ciphertext),
		"algorithm":  "aes-ecb",
	})
}
