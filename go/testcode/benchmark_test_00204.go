package testcode

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/hmac"
	"crypto/rand"
	"crypto/sha256"
	"encoding/hex"
	"io"
	"net/http"
)

func BenchmarkTest00204(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Plaintext string `json:"plaintext"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	encKey := make([]byte, 32)
	macKey := make([]byte, 32)
	if _, err := io.ReadFull(rand.Reader, encKey); err != nil {
		http.Error(w, "key generation failed", http.StatusInternalServerError)
		return
	}
	if _, err := io.ReadFull(rand.Reader, macKey); err != nil {
		http.Error(w, "key generation failed", http.StatusInternalServerError)
		return
	}

	block, err := aes.NewCipher(encKey)
	if err != nil {
		http.Error(w, "cipher init failed", http.StatusInternalServerError)
		return
	}

	iv := make([]byte, aes.BlockSize)
	if _, err := io.ReadFull(rand.Reader, iv); err != nil {
		http.Error(w, "iv generation failed", http.StatusInternalServerError)
		return
	}

	plainBytes := []byte(req.Plaintext)
	stream := cipher.NewCTR(block, iv)
	ciphertext := make([]byte, len(plainBytes))
	stream.XORKeyStream(ciphertext, plainBytes)

	payload := append(iv, ciphertext...)

	mac := hmac.New(sha256.New, macKey)
	mac.Write(payload)
	tag := mac.Sum(nil)

	authenticated := append(payload, tag...)

	RespondJSON(w, http.StatusOK, map[string]string{
		"ciphertext": hex.EncodeToString(authenticated),
		"algorithm":  "aes-ctr-hmac-sha256",
	})
}
