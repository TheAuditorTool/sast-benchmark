package testcode

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/hex"
	"io"
	"net/http"
)

var benchmarkTest00823Key = []byte("0123456789abcdef")

func BenchmarkTest00823(w http.ResponseWriter, r *http.Request) {
	plaintext := []byte(r.FormValue("data"))
	for len(plaintext)%aes.BlockSize != 0 {
		plaintext = append(plaintext, 0)
	}
	block, _ := aes.NewCipher(benchmarkTest00823Key)
	ct := make([]byte, aes.BlockSize+len(plaintext))
	iv := ct[:aes.BlockSize]
	io.ReadFull(rand.Reader, iv)
	cipher.NewCBCEncrypter(block, iv).CryptBlocks(ct[aes.BlockSize:], plaintext)
	RespondJSON(w, http.StatusOK, map[string]string{"ct": hex.EncodeToString(ct)})
}
