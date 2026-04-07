package testcode

import (
	"crypto/aes"
	"crypto/cipher"
	"encoding/hex"
	"net/http"
)

var benchmarkTest00826Key = []byte("0123456789abcdef")

func benchmarkTest00826ECBEncrypt(block cipher.Block, data []byte) []byte {
	bs := block.BlockSize()
	for len(data)%bs != 0 {
		data = append(data, 0)
	}
	ct := make([]byte, len(data))
	for i := 0; i < len(data); i += bs {
		block.Encrypt(ct[i:i+bs], data[i:i+bs])
	}
	return ct
}

func BenchmarkTest00826(w http.ResponseWriter, r *http.Request) {
	block, _ := aes.NewCipher(benchmarkTest00826Key)
	ct := benchmarkTest00826ECBEncrypt(block, []byte(r.FormValue("data")))
	RespondJSON(w, http.StatusOK, map[string]string{"ct": hex.EncodeToString(ct)})
}
