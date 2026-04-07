package testcode

import (
	"crypto/cipher"
	"crypto/des"
	"encoding/hex"
	"net/http"
)

type benchmarkTest00818Encryptor interface {
	Encrypt(data []byte) []byte
}

type benchmarkTest00818DESEncryptor struct {
	block cipher.Block
	iv    []byte
}

func (e *benchmarkTest00818DESEncryptor) Encrypt(data []byte) []byte {
	for len(data)%des.BlockSize != 0 {
		data = append(data, 0)
	}
	ct := make([]byte, len(data))
	cipher.NewCBCEncrypter(e.block, e.iv).CryptBlocks(ct, data)
	return ct
}

func BenchmarkTest00818(w http.ResponseWriter, r *http.Request) {
	block, _ := des.NewCipher([]byte("8bytekey"))
	var enc benchmarkTest00818Encryptor = &benchmarkTest00818DESEncryptor{block: block, iv: []byte("initvect")}
	ct := enc.Encrypt([]byte(r.FormValue("data")))
	RespondJSON(w, http.StatusOK, map[string]string{"ct": hex.EncodeToString(ct)})
}
