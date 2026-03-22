package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"

	"github.com/google/uuid"
)

const benchmarkTest00524StoragePath = "/var/app/storage"

func BenchmarkTest00524(w http.ResponseWriter, r *http.Request) {
	file, header, err := r.FormFile("upload")
	if err != nil {
		http.Error(w, "no file uploaded", http.StatusBadRequest)
		return
	}
	defer file.Close()

	newName := uuid.New().String() + filepath.Ext(header.Filename)
	destPath := filepath.Join(benchmarkTest00524StoragePath, newName)

	if err := os.MkdirAll(benchmarkTest00524StoragePath, 0750); err != nil {
		http.Error(w, "storage setup failed", http.StatusInternalServerError)
		return
	}

	dst, err := os.Create(destPath)
	if err != nil {
		http.Error(w, "failed to save file", http.StatusInternalServerError)
		return
	}
	defer dst.Close()

	io.Copy(dst, file)

	_, err = DB.Exec(
		"INSERT INTO uploads (id, original_name, stored_path) VALUES (?, ?, ?)",
		newName, header.Filename, destPath,
	)
	if err != nil {
		http.Error(w, "database error", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"status": "uploaded",
		"id":     newName,
	})
}
