package testcode

import (
	"crypto/sha256"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00811(w http.ResponseWriter, r *http.Request) {
	leaf := r.FormValue("data")
	prefix := []byte{0x00}
	h := sha256.Sum256(append(prefix, []byte(leaf)...))
	RespondJSON(w, http.StatusOK, map[string]string{"leaf_hash": hex.EncodeToString(h[:])})
}
