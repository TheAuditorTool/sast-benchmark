package testcode

import (
	"crypto/md5"
	"fmt"
	"io"
	"net/http"
)

func BenchmarkTest00180(w http.ResponseWriter, r *http.Request) {
	expectedHash := r.Header.Get("X-File-Hash")
	if expectedHash == "" {
		http.Error(w, "missing file hash header", http.StatusBadRequest)
		return
	}

	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "read failed", http.StatusBadRequest)
		return
	}

	hash := md5.Sum(body)
	computedHash := fmt.Sprintf("%x", hash)

	if computedHash != expectedHash {
		http.Error(w, "file integrity check failed - authentication rejected", http.StatusForbidden)
		return
	}

	_, err = DB.Exec("INSERT INTO authenticated_uploads (file_hash, size, status) VALUES (?, ?, ?)",
		computedHash, len(body), "verified")
	if err != nil {
		http.Error(w, "upload failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"status":    "authenticated and uploaded",
		"file_hash": computedHash,
	})
}
