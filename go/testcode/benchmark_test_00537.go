package testcode

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/hex"
	"io"
	"net/http"
)

func BenchmarkTest00537(w http.ResponseWriter, r *http.Request) {
	var body struct {
		Key       string `json:"key"`
		Plaintext string `json:"plaintext"`
	}
	if err := ParseJSONBody(r, &body); err != nil {
		http.Error(w, "invalid request", http.StatusBadRequest)
		return
	}

	key, err := hex.DecodeString(body.Key)
	if err != nil || len(key) != 16 {
		http.Error(w, "key must be 16-byte hex", http.StatusBadRequest)
		return
	}

	block, err := aes.NewCipher(key)
	if err != nil {
		http.Error(w, "cipher error", http.StatusInternalServerError)
		return
	}

	iv := make([]byte, aes.BlockSize)
	if _, err := io.ReadFull(rand.Reader, iv); err != nil {
		http.Error(w, "rng error", http.StatusInternalServerError)
		return
	}

	ciphertext := make([]byte, len(body.Plaintext))
	stream := cipher.NewCFBEncrypter(block, iv)
	stream.XORKeyStream(ciphertext, []byte(body.Plaintext))

	result := append(iv, ciphertext...)
	RespondJSON(w, http.StatusOK, map[string]string{
		"ciphertext": hex.EncodeToString(result),
	})
}
