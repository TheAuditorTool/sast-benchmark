package testcode

import (
	"crypto/sha256"
	"fmt"
	"io"
	"net/http"
)

func BenchmarkTest00801(w http.ResponseWriter, r *http.Request) {
	h := sha256.New()
	io.Copy(h, r.Body)
	key := fmt.Sprintf("%x", h.Sum(nil))
	RespondJSON(w, http.StatusOK, map[string]string{"dedup_key": key})
}
