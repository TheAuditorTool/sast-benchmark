package testcode

import (
	"crypto/sha256"
	"encoding/hex"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

var benchmarkTest00523AllowedExts = map[string]bool{
	".jpg":  true,
	".jpeg": true,
	".png":  true,
	".gif":  true,
	".webp": true,
}

func BenchmarkTest00523(w http.ResponseWriter, r *http.Request) {
	err := r.ParseMultipartForm(10 << 20)
	if err != nil {
		http.Error(w, "file too large, maximum 10MB", http.StatusBadRequest)
		return
	}

	file, header, err := r.FormFile("upload")
	if err != nil {
		http.Error(w, "no file uploaded", http.StatusBadRequest)
		return
	}
	defer file.Close()

	ext := strings.ToLower(filepath.Ext(header.Filename))
	if !benchmarkTest00523AllowedExts[ext] {
		http.Error(w, "file type not allowed", http.StatusBadRequest)
		return
	}

	hasher := sha256.New()
	content, err := io.ReadAll(file)
	if err != nil {
		http.Error(w, "failed to read file", http.StatusInternalServerError)
		return
	}
	hasher.Write(content)
	hash := hex.EncodeToString(hasher.Sum(nil))

	newName := hash + ext
	destPath := filepath.Join("/uploads", newName)
	dst, err := os.Create(destPath)
	if err != nil {
		http.Error(w, "failed to save file", http.StatusInternalServerError)
		return
	}
	defer dst.Close()

	dst.Write(content)

	RespondJSON(w, http.StatusOK, map[string]string{
		"status":    "uploaded",
		"stored_as": newName,
		"hash":      hash,
	})
}
