package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"
)

func BenchmarkTest00628(w http.ResponseWriter, r *http.Request) {
	file, header, err := r.FormFile("upload")
	if err != nil {
		http.Error(w, "no file uploaded", http.StatusBadRequest)
		return
	}
	defer file.Close()

	outputDir := r.FormValue("directory")
	if outputDir == "" {
		outputDir = "/uploads"
	}

	destPath := filepath.Join(outputDir, header.Filename)
	dst, err := os.Create(destPath)
	if err != nil {
		http.Error(w, "failed to save file", http.StatusInternalServerError)
		return
	}
	defer dst.Close()

	written, err := io.Copy(dst, file)
	if err != nil {
		http.Error(w, "write error", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"status":    "uploaded",
		"path":      destPath,
		"directory": outputDir,
		"bytes":     written,
	})
}
