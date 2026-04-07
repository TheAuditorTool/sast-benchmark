package testcode

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/hex"
	"io"
	"net/http"
)

type benchmarkTest00839GCMEncryptor struct {
	key []byte
}

func (e *benchmarkTest00839GCMEncryptor) Encrypt(pt []byte) ([]byte, error) {
	block, err := aes.NewCipher(e.key)
	if err != nil {
		return nil, err
	}
	gcm, _ := cipher.NewGCM(block)
	nonce := make([]byte, gcm.NonceSize())
	io.ReadFull(rand.Reader, nonce)
	return gcm.Seal(nonce, nonce, pt, nil), nil
}

var benchmarkTest00839Enc = &benchmarkTest00839GCMEncryptor{key: []byte("0123456789abcdef0123456789abcdef")}

func BenchmarkTest00839(w http.ResponseWriter, r *http.Request) {
	ct, err := benchmarkTest00839Enc.Encrypt([]byte(r.FormValue("data")))
	if err != nil {
		http.Error(w, "encrypt error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"ct": hex.EncodeToString(ct)})
}
