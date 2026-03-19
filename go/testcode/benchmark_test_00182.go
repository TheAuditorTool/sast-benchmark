package testcode

import (
	"crypto/sha512"
	"fmt"
	"io"
	"net/http"
)

func BenchmarkTest00182(w http.ResponseWriter, r *http.Request) {
	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "read failed", http.StatusBadRequest)
		return
	}

	hash := sha512.Sum512(body)
	hashStr := fmt.Sprintf("%x", hash)

	_, err = DB.Exec("INSERT INTO document_hashes (hash, algorithm, size) VALUES (?, ?, ?)",
		hashStr, "sha512", len(body))
	if err != nil {
		http.Error(w, "storage failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"hash":      hashStr,
		"algorithm": "sha512",
	})
}
