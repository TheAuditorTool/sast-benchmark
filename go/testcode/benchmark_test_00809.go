package testcode

import (
	"crypto/sha256"
	"fmt"
	"io"
	"net/http"
)

func BenchmarkTest00809(w http.ResponseWriter, r *http.Request) {
	data, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "read error", http.StatusBadRequest)
		return
	}
	h := sha256.Sum256(data)
	etag := fmt.Sprintf(`"%x"`, h)
	w.Header().Set("ETag", etag)
	w.Header().Set("Content-Type", "application/octet-stream")
	w.Write(data)
}
