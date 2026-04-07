package testcode

import (
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha256"
	"encoding/hex"
	"net/http"
)

var benchmarkTest00834Key, _ = rsa.GenerateKey(rand.Reader, 2048)

func BenchmarkTest00834(w http.ResponseWriter, r *http.Request) {
	plaintext := []byte(r.FormValue("data"))
	ct, err := rsa.EncryptOAEP(sha256.New(), rand.Reader, &benchmarkTest00834Key.PublicKey, plaintext, nil)
	if err != nil {
		http.Error(w, "encrypt error", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"ct": hex.EncodeToString(ct)})
}
