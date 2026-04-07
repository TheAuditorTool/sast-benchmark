package testcode

import (
	"crypto/sha256"
	"encoding/hex"
	"io"
	"net/http"

	"golang.org/x/crypto/hkdf"
)

func BenchmarkTest00805(w http.ResponseWriter, r *http.Request) {
	secret := r.FormValue("secret")
	salt := r.FormValue("salt")
	hk := hkdf.New(sha256.New, []byte(secret), []byte(salt), []byte("app-context"))
	key := make([]byte, 32)
	io.ReadFull(hk, key)
	RespondJSON(w, http.StatusOK, map[string]string{"derived_key": hex.EncodeToString(key)})
}
