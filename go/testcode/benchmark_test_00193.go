package testcode

import (
	"crypto/rc4"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00193(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Plaintext string `json:"plaintext"`
		Key       string `json:"key"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	cipher, err := rc4.NewCipher([]byte(req.Key))
	if err != nil {
		http.Error(w, "cipher init failed", http.StatusInternalServerError)
		return
	}

	ciphertext := make([]byte, len(req.Plaintext))
	cipher.XORKeyStream(ciphertext, []byte(req.Plaintext))

	RespondJSON(w, http.StatusOK, map[string]string{
		"ciphertext": hex.EncodeToString(ciphertext),
		"algorithm":  "rc4",
	})
}
