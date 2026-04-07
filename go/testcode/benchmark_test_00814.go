package testcode

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/hex"
	"io"
	"net/http"
)

var benchmarkTest00814Key = []byte("0123456789abcdef")

func BenchmarkTest00814(w http.ResponseWriter, r *http.Request) {
	plaintext := []byte(r.FormValue("data"))
	block, _ := aes.NewCipher(benchmarkTest00814Key)
	ct := make([]byte, aes.BlockSize+len(plaintext))
	iv := ct[:aes.BlockSize]
	io.ReadFull(rand.Reader, iv)
	cipher.NewCFBEncrypter(block, iv).XORKeyStream(ct[aes.BlockSize:], plaintext)
	RespondJSON(w, http.StatusOK, map[string]string{"ct": hex.EncodeToString(ct)})
}
