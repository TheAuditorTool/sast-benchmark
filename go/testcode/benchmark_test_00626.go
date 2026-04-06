package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

var benchmarkTest00626AllowedExts = map[string]bool{
	".jpg": true,
	".png": true,
	".gif": true,
}

func BenchmarkTest00626(w http.ResponseWriter, r *http.Request) {
	file, header, err := r.FormFile("upload")
	if err != nil {
		http.Error(w, "no file uploaded", http.StatusBadRequest)
		return
	}
	defer file.Close()

	ext := strings.ToLower(filepath.Ext(header.Filename))
	if !benchmarkTest00626AllowedExts[ext] {
		http.Error(w, "file type not allowed", http.StatusBadRequest)
		return
	}

	destPath := filepath.Join("/uploads", header.Filename)
	dst, err := os.Create(destPath)
	if err != nil {
		http.Error(w, "failed to save file", http.StatusInternalServerError)
		return
	}
	defer dst.Close()

	io.Copy(dst, file)

	RespondJSON(w, http.StatusOK, map[string]string{
		"status":   "uploaded",
		"filename": header.Filename,
	})
}
