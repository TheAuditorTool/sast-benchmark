package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

func BenchmarkTest00518(w http.ResponseWriter, r *http.Request) {
	file, header, err := r.FormFile("upload")
	if err != nil {
		http.Error(w, "no file uploaded", http.StatusBadRequest)
		return
	}
	defer file.Close()

	contentType := header.Header.Get("Content-Type")
	if !strings.HasPrefix(contentType, "image/") {
		http.Error(w, "only image uploads are allowed", http.StatusBadRequest)
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
		"status":       "uploaded",
		"filename":     header.Filename,
		"content_type": contentType,
	})
}
