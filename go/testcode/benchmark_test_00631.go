package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"

	"github.com/google/uuid"
)

var benchmarkTest00631AllowedMIMEs = map[string]bool{
	"image/jpeg": true,
	"image/png":  true,
	"image/gif":  true,
}

func BenchmarkTest00631(w http.ResponseWriter, r *http.Request) {
	file, header, err := r.FormFile("upload")
	if err != nil {
		http.Error(w, "no file uploaded", http.StatusBadRequest)
		return
	}
	defer file.Close()

	buf := make([]byte, 512)
	n, err := file.Read(buf)
	if err != nil && err != io.EOF {
		http.Error(w, "failed to read file", http.StatusInternalServerError)
		return
	}

	detectedMIME := http.DetectContentType(buf[:n])
	if !benchmarkTest00631AllowedMIMEs[detectedMIME] {
		http.Error(w, "file type not allowed", http.StatusBadRequest)
		return
	}

	if _, err := file.Seek(0, 0); err != nil {
		http.Error(w, "failed to process file", http.StatusInternalServerError)
		return
	}

	ext := filepath.Ext(header.Filename)
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
		"status":        "uploaded",
		"stored_as":     newName,
		"detected_mime": detectedMIME,
	})
}
