package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"io"
	"net/http"

	"golang.org/x/crypto/chacha20poly1305"
)

var benchmarkTest00540Key = []byte{
	0x1a, 0x2b, 0x3c, 0x4d, 0x5e, 0x6f, 0x7a, 0x8b,
	0x9c, 0xad, 0xbe, 0xcf, 0xd0, 0xe1, 0xf2, 0x03,
	0x14, 0x25, 0x36, 0x47, 0x58, 0x69, 0x7a, 0x8b,
	0x9c, 0xad, 0xbe, 0xcf, 0xd0, 0xe1, 0xf2, 0x03,
}

func BenchmarkTest00540(w http.ResponseWriter, r *http.Request) {
	var body struct {
		Plaintext string `json:"plaintext"`
	}
	if err := ParseJSONBody(r, &body); err != nil {
		http.Error(w, "invalid request", http.StatusBadRequest)
		return
	}

	aead, err := chacha20poly1305.NewX(benchmarkTest00540Key)
	if err != nil {
		http.Error(w, "cipher init error", http.StatusInternalServerError)
		return
	}

	nonce := make([]byte, chacha20poly1305.NonceSizeX)
	if _, err := io.ReadFull(rand.Reader, nonce); err != nil {
		http.Error(w, "rng error", http.StatusInternalServerError)
		return
	}

	sealed := aead.Seal(nonce, nonce, []byte(body.Plaintext), nil)
	RespondJSON(w, http.StatusOK, map[string]string{
		"ciphertext": hex.EncodeToString(sealed),
	})
}
