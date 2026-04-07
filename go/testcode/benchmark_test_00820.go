package testcode

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/hex"
	"io"
	"net/http"
	"sync"
)

var benchmarkTest00820Key = []byte("0123456789abcdef")

func BenchmarkTest00820(w http.ResponseWriter, r *http.Request) {
	plaintext := []byte(r.FormValue("data"))
	var ct []byte
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		block, _ := aes.NewCipher(benchmarkTest00820Key)
		ciphertext := make([]byte, aes.BlockSize+len(plaintext))
		iv := ciphertext[:aes.BlockSize]
		io.ReadFull(rand.Reader, iv)
		cipher.NewCTR(block, iv).XORKeyStream(ciphertext[aes.BlockSize:], plaintext)
		ct = ciphertext
	}()
	wg.Wait()
	RespondJSON(w, http.StatusOK, map[string]string{"ct": hex.EncodeToString(ct)})
}
