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

var benchmarkTest00836Key = []byte("0123456789abcdef0123456789abcdef")

func BenchmarkTest00836(w http.ResponseWriter, r *http.Request) {
	plaintext := []byte(r.FormValue("data"))
	var ct []byte
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		block, _ := aes.NewCipher(benchmarkTest00836Key)
		gcm, _ := cipher.NewGCM(block)
		nonce := make([]byte, gcm.NonceSize())
		io.ReadFull(rand.Reader, nonce)
		ct = gcm.Seal(nonce, nonce, plaintext, nil)
	}()
	wg.Wait()
	RespondJSON(w, http.StatusOK, map[string]string{"ct": hex.EncodeToString(ct)})
}
