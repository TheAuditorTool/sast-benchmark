package testcode

import (
	"encoding/hex"
	"io"
	"net/http"

	"golang.org/x/crypto/blake2b"
)

func BenchmarkTest00812(w http.ResponseWriter, r *http.Request) {
	data, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "read error", http.StatusBadRequest)
		return
	}
	h, _ := blake2b.New256(nil)
	h.Write(data)
	fingerprint := hex.EncodeToString(h.Sum(nil))
	RespondJSON(w, http.StatusOK, map[string]string{"fingerprint": fingerprint})
}
