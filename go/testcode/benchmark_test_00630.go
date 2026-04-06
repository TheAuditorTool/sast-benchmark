package testcode

import (
	"crypto/sha256"
	"encoding/hex"
	"io"
	"net/http"
	"os"
	"path/filepath"
)

func benchmarkTest00630HashFile(path string) (string, error) {
	f, err := os.Open(path)
	if err != nil {
		return "", err
	}
	defer f.Close()
	h := sha256.New()
	io.Copy(h, f)
	return hex.EncodeToString(h.Sum(nil)), nil
}

func BenchmarkTest00630(w http.ResponseWriter, r *http.Request) {
	file, header, err := r.FormFile("upload")
	if err != nil {
		http.Error(w, "no file uploaded", http.StatusBadRequest)
		return
	}
	defer file.Close()

	destPath := filepath.Join("/uploads", header.Filename)

	previousHash := ""
	if _, statErr := os.Stat(destPath); statErr == nil {
		previousHash, _ = benchmarkTest00630HashFile(destPath)
	}

	dst, err := os.Create(destPath)
	if err != nil {
		http.Error(w, "failed to save file", http.StatusInternalServerError)
		return
	}
	defer dst.Close()

	io.Copy(dst, file)
	dst.Close()

	newHash, _ := benchmarkTest00630HashFile(destPath)

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"status":        "uploaded",
		"filename":      header.Filename,
		"previous_hash": previousHash,
		"current_hash":  newHash,
		"overwritten":   previousHash != "" && previousHash != newHash,
	})
}
