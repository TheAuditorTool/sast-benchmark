package testcode

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/hex"
	"io"
	"net/http"
)

var benchmarkTest00837Key = []byte("0123456789abcdef0123456789abcdef")

func BenchmarkTest00837(w http.ResponseWriter, r *http.Request) {
	plaintext := []byte(r.FormValue("data"))
	aad := []byte(r.Header.Get("X-Request-ID"))
	block, _ := aes.NewCipher(benchmarkTest00837Key)
	gcm, _ := cipher.NewGCM(block)
	nonce := make([]byte, gcm.NonceSize())
	io.ReadFull(rand.Reader, nonce)
	ct := gcm.Seal(nonce, nonce, plaintext, aad)
	RespondJSON(w, http.StatusOK, map[string]string{"ct": hex.EncodeToString(ct)})
}
