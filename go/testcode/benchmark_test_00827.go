package testcode

import (
	"crypto/cipher"
	"crypto/des"
	"encoding/hex"
	"net/http"
)

var benchmarkTest00827Key = []byte("012345678901234567890123")
var benchmarkTest00827IV = []byte("initvect")

func BenchmarkTest00827(w http.ResponseWriter, r *http.Request) {
	plaintext := []byte(r.FormValue("data"))
	for len(plaintext)%des.BlockSize != 0 {
		plaintext = append(plaintext, 0)
	}
	block, _ := des.NewTripleDESCipher(benchmarkTest00827Key)
	ct := make([]byte, len(plaintext))
	cipher.NewCBCEncrypter(block, benchmarkTest00827IV).CryptBlocks(ct, plaintext)
	RespondJSON(w, http.StatusOK, map[string]string{"ct": hex.EncodeToString(ct)})
}
