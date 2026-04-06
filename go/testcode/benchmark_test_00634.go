package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"

	"github.com/google/uuid"
)

var benchmarkTest00634AllowedExts = map[string]bool{
	".jpg":  true,
	".jpeg": true,
	".png":  true,
	".gif":  true,
	".webp": true,
}

func BenchmarkTest00634(w http.ResponseWriter, r *http.Request) {
	file, header, err := r.FormFile("upload")
	if err != nil {
		http.Error(w, "no file uploaded", http.StatusBadRequest)
		return
	}
	defer file.Close()

	ext := strings.ToLower(filepath.Ext(header.Filename))
	if !benchmarkTest00634AllowedExts[ext] {
		http.Error(w, "file type not allowed", http.StatusBadRequest)
		return
	}

	dir, err := os.MkdirTemp("/uploads", "upload-")
	if err != nil {
		http.Error(w, "failed to create upload directory", http.StatusInternalServerError)
		return
	}

	newName := uuid.New().String() + ext
	destPath := filepath.Join(dir, newName)

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
		"directory": filepath.Base(dir),
		"bytes":     written,
	})
}
