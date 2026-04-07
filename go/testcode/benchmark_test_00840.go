package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"io"
	"net/http"

	"golang.org/x/crypto/nacl/secretbox"
)

var benchmarkTest00840Key [32]byte

func init() {
	io.ReadFull(rand.Reader, benchmarkTest00840Key[:])
}

func BenchmarkTest00840(w http.ResponseWriter, r *http.Request) {
	plaintext := []byte(r.FormValue("data"))
	var nonce [24]byte
	io.ReadFull(rand.Reader, nonce[:])
	ct := secretbox.Seal(nonce[:], plaintext, &nonce, &benchmarkTest00840Key)
	RespondJSON(w, http.StatusOK, map[string]string{"ct": hex.EncodeToString(ct)})
}
