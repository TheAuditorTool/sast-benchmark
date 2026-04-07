package testcode

import (
	"crypto/aes"
	"crypto/cipher"
	"encoding/hex"
	"net/http"
)

var benchmarkTest00835Key = []byte("0123456789abcdef0123456789abcdef")

func BenchmarkTest00835(w http.ResponseWriter, r *http.Request) {
	ctHex := r.FormValue("ct")
	ctBytes, err := hex.DecodeString(ctHex)
	if err != nil {
		http.Error(w, "invalid ct", http.StatusBadRequest)
		return
	}
	block, _ := aes.NewCipher(benchmarkTest00835Key)
	gcm, _ := cipher.NewGCM(block)
	if len(ctBytes) < gcm.NonceSize() {
		http.Error(w, "ct too short", http.StatusBadRequest)
		return
	}
	nonce, ciphertext := ctBytes[:gcm.NonceSize()], ctBytes[gcm.NonceSize():]
	plaintext, err := gcm.Open(nil, nonce, ciphertext, nil)
	if err != nil {
		http.Error(w, "decryption failed", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"data": string(plaintext)})
}
