package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"
)

func BenchmarkTest00625(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPut {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	filename := r.Header.Get("X-Filename")
	if filename == "" {
		http.Error(w, "X-Filename header required", http.StatusBadRequest)
		return
	}

	destPath := filepath.Join("/uploads", filename)
	dst, err := os.Create(destPath)
	if err != nil {
		http.Error(w, "failed to create file", http.StatusInternalServerError)
		return
	}
	defer dst.Close()

	written, err := io.Copy(dst, r.Body)
	if err != nil {
		http.Error(w, "failed to write file", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"status":   "uploaded",
		"filename": filename,
		"bytes":    written,
	})
}
