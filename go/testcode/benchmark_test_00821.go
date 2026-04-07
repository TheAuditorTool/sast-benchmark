package testcode

import (
	"crypto/cipher"
	"crypto/des"
	"encoding/hex"
	"net/http"
)

func benchmarkTest00821Encrypt(data, key []byte) []byte {
	for len(data)%des.BlockSize != 0 {
		data = append(data, 0)
	}
	block, _ := des.NewCipher(key)
	ct := make([]byte, len(data))
	iv := make([]byte, des.BlockSize)
	cipher.NewCBCEncrypter(block, iv).CryptBlocks(ct, data)
	return ct
}

func BenchmarkTest00821(w http.ResponseWriter, r *http.Request) {
	ct := benchmarkTest00821Encrypt([]byte(r.FormValue("data")), []byte("8bytekey"))
	RespondJSON(w, http.StatusOK, map[string]string{"ct": hex.EncodeToString(ct)})
}
