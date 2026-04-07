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

var benchmarkTest00833EncKey = []byte("0123456789abcdef0123456789abcdef")
var benchmarkTest00833MacKey = []byte("fedcba9876543210fedcba9876543210")

func BenchmarkTest00833(w http.ResponseWriter, r *http.Request) {
	plaintext := []byte(r.FormValue("data"))
	block, _ := aes.NewCipher(benchmarkTest00833EncKey)
	ct := make([]byte, aes.BlockSize+len(plaintext))
	iv := ct[:aes.BlockSize]
	io.ReadFull(rand.Reader, iv)
	cipher.NewCTR(block, iv).XORKeyStream(ct[aes.BlockSize:], plaintext)
	mac := hmac.New(sha256.New, benchmarkTest00833MacKey)
	mac.Write(ct)
	tag := mac.Sum(nil)
	RespondJSON(w, http.StatusOK, map[string]string{
		"ct":  hex.EncodeToString(ct),
		"mac": hex.EncodeToString(tag),
	})
}
