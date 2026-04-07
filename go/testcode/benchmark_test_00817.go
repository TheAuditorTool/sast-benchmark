package testcode

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/hex"
	"io"
	"net/http"
)

var benchmarkTest00817Key = []byte("0123456789abcdef")

func BenchmarkTest00817(w http.ResponseWriter, r *http.Request) {
	plaintext := []byte(r.FormValue("data"))
	block, _ := aes.NewCipher(benchmarkTest00817Key)
	ct := make([]byte, aes.BlockSize+len(plaintext))
	iv := ct[:aes.BlockSize]
	io.ReadFull(rand.Reader, iv)
	cipher.NewOFB(block, iv).XORKeyStream(ct[aes.BlockSize:], plaintext)
	RespondJSON(w, http.StatusOK, map[string]string{"ct": hex.EncodeToString(ct)})
}
