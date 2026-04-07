package testcode

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/hex"
	"io"
	"net/http"
	"os"
)

func BenchmarkTest00842(w http.ResponseWriter, r *http.Request) {
	keyHex := os.Getenv("AES_KEY")
	key, err := hex.DecodeString(keyHex)
	if err != nil || (len(key) != 16 && len(key) != 24 && len(key) != 32) {
		http.Error(w, "key error", http.StatusInternalServerError)
		return
	}
	block, _ := aes.NewCipher(key)
	gcm, _ := cipher.NewGCM(block)
	nonce := make([]byte, 12)
	io.ReadFull(rand.Reader, nonce)
	ct := gcm.Seal(nonce, nonce, []byte(r.FormValue("data")), nil)
	RespondJSON(w, http.StatusOK, map[string]string{"ct": hex.EncodeToString(ct)})
}
