package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"

	"github.com/google/uuid"
)

var benchmarkTest00522MagicBytes = map[string][]byte{
	"jpeg": {0xFF, 0xD8, 0xFF},
	"png":  {0x89, 0x50, 0x4E, 0x47},
	"gif":  {0x47, 0x49, 0x46, 0x38},
}

func benchmarkTest00522MatchesMagic(header []byte) bool {
	for _, magic := range benchmarkTest00522MagicBytes {
		if len(header) >= len(magic) {
			match := true
			for i, b := range magic {
				if header[i] != b {
					match = false
					break
				}
			}
			if match {
				return true
			}
		}
	}
	return false
}

func BenchmarkTest00522(w http.ResponseWriter, r *http.Request) {
	file, header, err := r.FormFile("upload")
	if err != nil {
		http.Error(w, "no file uploaded", http.StatusBadRequest)
		return
	}
	defer file.Close()

	magicBuf := make([]byte, 4)
	n, err := file.Read(magicBuf)
	if err != nil || n < 3 {
		http.Error(w, "cannot read file header", http.StatusBadRequest)
		return
	}

	if !benchmarkTest00522MatchesMagic(magicBuf[:n]) {
		http.Error(w, "invalid file type: magic bytes do not match any allowed image format", http.StatusBadRequest)
		return
	}

	file.Seek(0, 0)

	newName := uuid.New().String() + filepath.Ext(header.Filename)
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
