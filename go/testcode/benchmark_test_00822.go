package testcode

import (
	"context"
	"crypto/aes"
	"crypto/cipher"
	"encoding/hex"
	"net/http"
)

type benchmarkTest00822CtxKey struct{}

func BenchmarkTest00822(w http.ResponseWriter, r *http.Request) {
	key := []byte("0123456789abcdef")
	ctx := context.WithValue(r.Context(), benchmarkTest00822CtxKey{}, key)
	k := ctx.Value(benchmarkTest00822CtxKey{}).([]byte)
	plaintext := []byte(r.FormValue("data"))
	for len(plaintext)%aes.BlockSize != 0 {
		plaintext = append(plaintext, 0)
	}
	block, _ := aes.NewCipher(k)
	iv := make([]byte, aes.BlockSize)
	ct := make([]byte, len(plaintext))
	cipher.NewCBCEncrypter(block, iv).CryptBlocks(ct, plaintext)
	RespondJSON(w, http.StatusOK, map[string]string{"ct": hex.EncodeToString(ct)})
}
