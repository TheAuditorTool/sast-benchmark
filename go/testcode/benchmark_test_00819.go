package testcode

import (
	"crypto/cipher"
	"crypto/des"
	"encoding/hex"
	"net/http"
)

type benchmarkTest00819Encryptor struct {
	block cipher.Block
	iv    []byte
}

func (e *benchmarkTest00819Encryptor) Run(data []byte) []byte {
	for len(data)%e.block.BlockSize() != 0 {
		data = append(data, 0)
	}
	ct := make([]byte, len(data))
	cipher.NewCBCEncrypter(e.block, e.iv).CryptBlocks(ct, data)
	return ct
}

var benchmarkTest00819Enc = func() *benchmarkTest00819Encryptor {
	b, _ := des.NewCipher([]byte("8bytekey"))
	return &benchmarkTest00819Encryptor{block: b, iv: []byte("initvect")}
}()

func BenchmarkTest00819(w http.ResponseWriter, r *http.Request) {
	ct := benchmarkTest00819Enc.Run([]byte(r.FormValue("data")))
	RespondJSON(w, http.StatusOK, map[string]string{"ct": hex.EncodeToString(ct)})
}
