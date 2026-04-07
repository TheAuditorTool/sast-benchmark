package testcode

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"crypto/sha256"
	"encoding/hex"
	"io"
	"net/http"
	"os"

	"golang.org/x/crypto/hkdf"
)

func BenchmarkTest00832(w http.ResponseWriter, r *http.Request) {
	masterKey := []byte(os.Getenv("MASTER_KEY"))
	hk := hkdf.New(sha256.New, masterKey, nil, []byte("encrypt-v1"))
	key := make([]byte, 32)
	io.ReadFull(hk, key)
	block, _ := aes.NewCipher(key)
	gcm, _ := cipher.NewGCM(block)
	nonce := make([]byte, gcm.NonceSize())
	io.ReadFull(rand.Reader, nonce)
	ct := gcm.Seal(nonce, nonce, []byte(r.FormValue("data")), nil)
	RespondJSON(w, http.StatusOK, map[string]string{"ct": hex.EncodeToString(ct)})
}
