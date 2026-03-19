package testcode

import (
	"crypto/sha256"
	"fmt"
	"io"
	"net/http"
)

func BenchmarkTest00187(w http.ResponseWriter, r *http.Request) {
	file, header, err := r.FormFile("upload")
	if err != nil {
		http.Error(w, "file required", http.StatusBadRequest)
		return
	}
	defer file.Close()

	h := sha256.New()
	size, err := io.Copy(h, file)
	if err != nil {
		http.Error(w, "checksum failed", http.StatusInternalServerError)
		return
	}

	checksum := fmt.Sprintf("%x", h.Sum(nil))

	_, err = DB.Exec("INSERT INTO uploads (filename, checksum, size) VALUES (?, ?, ?)",
		header.Filename, checksum, size)
	if err != nil {
		http.Error(w, "storage failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"filename": header.Filename,
		"checksum": checksum,
	})
}
