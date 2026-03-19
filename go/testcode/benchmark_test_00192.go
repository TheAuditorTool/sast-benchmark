package testcode

import (
	"crypto/des"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00192(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Plaintext string `json:"plaintext"`
		Key       string `json:"key"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	keyBytes := []byte(req.Key)
	if len(keyBytes) < 24 {
		keyBytes = append(keyBytes, make([]byte, 24-len(keyBytes))...)
	}

	block, err := des.NewTripleDESCipher(keyBytes[:24])
	if err != nil {
		http.Error(w, "cipher init failed", http.StatusInternalServerError)
		return
	}

	plainBytes := []byte(req.Plaintext)
	padded := make([]byte, ((len(plainBytes)/8)+1)*8)
	copy(padded, plainBytes)

	ciphertext := make([]byte, len(padded))
	for i := 0; i < len(padded); i += 8 {
		block.Encrypt(ciphertext[i:i+8], padded[i:i+8])
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"ciphertext": hex.EncodeToString(ciphertext),
		"algorithm":  "3des",
	})
}
