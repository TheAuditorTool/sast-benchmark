package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"

	"github.com/google/uuid"
)

var benchmarkTest00632AllowedExts = map[string]bool{
	".jpg":  true,
	".jpeg": true,
	".png":  true,
	".gif":  true,
	".webp": true,
}

func BenchmarkTest00632(w http.ResponseWriter, r *http.Request) {
	r.Body = http.MaxBytesReader(w, r.Body, 5<<20)

	file, header, err := r.FormFile("upload")
	if err != nil {
		http.Error(w, "upload failed or file exceeds 5MB limit", http.StatusBadRequest)
		return
	}
	defer file.Close()

	ext := strings.ToLower(filepath.Ext(header.Filename))
	if !benchmarkTest00632AllowedExts[ext] {
		http.Error(w, "file type not allowed", http.StatusBadRequest)
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

	written, err := io.Copy(dst, file)
	if err != nil {
		http.Error(w, "failed to write file", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"status":    "uploaded",
		"stored_as": newName,
		"bytes":     written,
	})
}
