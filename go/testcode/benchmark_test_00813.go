package testcode

import (
	"crypto/aes"
	"crypto/cipher"
	"encoding/hex"
	"net/http"
)

var benchmarkTest00813Key = []byte("0123456789abcdef")
var benchmarkTest00813IV = []byte("staticivstaticiv")

func BenchmarkTest00813(w http.ResponseWriter, r *http.Request) {
	plaintext := []byte(r.FormValue("data"))
	for len(plaintext)%aes.BlockSize != 0 {
		plaintext = append(plaintext, 0)
	}
	block, _ := aes.NewCipher(benchmarkTest00813Key)
	ct := make([]byte, len(plaintext))
	cipher.NewCBCEncrypter(block, benchmarkTest00813IV).CryptBlocks(ct, plaintext)
	RespondJSON(w, http.StatusOK, map[string]string{"ct": hex.EncodeToString(ct)})
}
