package testcode

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/hex"
	"io"
	"net/http"
)

var benchmarkTest00828Key = []byte("0123456789abcdef0123456789abcdef")

func BenchmarkTest00828(w http.ResponseWriter, r *http.Request) {
	plaintext := []byte(r.FormValue("data"))
	block, _ := aes.NewCipher(benchmarkTest00828Key)
	gcm, _ := cipher.NewGCM(block)
	nonce := make([]byte, gcm.NonceSize())
	io.ReadFull(rand.Reader, nonce)
	ct := gcm.Seal(nonce, nonce, plaintext, nil)
	RespondJSON(w, http.StatusOK, map[string]string{"ct": hex.EncodeToString(ct)})
}
