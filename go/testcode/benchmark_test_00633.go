package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"

	"github.com/google/uuid"
)

var benchmarkTest00633AllowedExts = map[string]bool{
	".jpg":  true,
	".jpeg": true,
	".png":  true,
	".gif":  true,
}

var benchmarkTest00633AllowedMIMEs = map[string]bool{
	"image/jpeg": true,
	"image/png":  true,
	"image/gif":  true,
}

func BenchmarkTest00633(w http.ResponseWriter, r *http.Request) {
	file, header, err := r.FormFile("upload")
	if err != nil {
		http.Error(w, "no file uploaded", http.StatusBadRequest)
		return
	}
	defer file.Close()

	ext := strings.ToLower(filepath.Ext(header.Filename))
	if !benchmarkTest00633AllowedExts[ext] {
		http.Error(w, "file extension not allowed", http.StatusBadRequest)
		return
	}

	buf := make([]byte, 512)
	n, err := file.Read(buf)
	if err != nil && err != io.EOF {
		http.Error(w, "failed to read file", http.StatusInternalServerError)
		return
	}

	detectedMIME := http.DetectContentType(buf[:n])
	if !benchmarkTest00633AllowedMIMEs[detectedMIME] {
		http.Error(w, "file content does not match allowed image types", http.StatusBadRequest)
		return
	}

	if _, err := file.Seek(0, 0); err != nil {
		http.Error(w, "failed to process file", http.StatusInternalServerError)
		return
	}

	newName := uuid.New().String() + ext
	destPath := filepath.Join("/uploads", newName)

	dst, err := os.Create(destPath)
	if err != nil {
		http.Error(w, "failed to save file", http.StatusInternalServerError)
		return
	}
	defer dst.Close()

	io.Copy(dst, file)

	RespondJSON(w, http.StatusOK, map[string]string{
		"status":    "uploaded",
		"stored_as": newName,
	})
}
