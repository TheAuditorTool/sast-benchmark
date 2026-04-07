package testcode

import (
	"crypto/cipher"
	"crypto/des"
	"encoding/hex"
	"net/http"
)

var benchmarkTest00816Key = []byte("8bytekey")
var benchmarkTest00816IV = []byte("initvect")

func BenchmarkTest00816(w http.ResponseWriter, r *http.Request) {
	plaintext := []byte(r.FormValue("data"))
	for len(plaintext)%des.BlockSize != 0 {
		plaintext = append(plaintext, 0)
	}
	block, _ := des.NewCipher(benchmarkTest00816Key)
	ct := make([]byte, len(plaintext))
	cipher.NewCBCEncrypter(block, benchmarkTest00816IV).CryptBlocks(ct, plaintext)
	RespondJSON(w, http.StatusOK, map[string]string{"ct": hex.EncodeToString(ct)})
}
