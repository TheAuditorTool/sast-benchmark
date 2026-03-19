package testcode

import (
	"crypto/sha256"
	"fmt"
	"io"
	"net/http"
)

func BenchmarkTest00181(w http.ResponseWriter, r *http.Request) {
	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "read failed", http.StatusBadRequest)
		return
	}

	hash := sha256.Sum256(body)
	integrity := fmt.Sprintf("%x", hash)

	_, err = DB.Exec("INSERT INTO file_checksums (checksum, size) VALUES (?, ?)",
		integrity, len(body))
	if err != nil {
		http.Error(w, "storage failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"integrity": integrity,
		"algorithm": "sha256",
	})
}
