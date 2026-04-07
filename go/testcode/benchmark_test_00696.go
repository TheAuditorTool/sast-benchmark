package testcode

import (
	"net/http"
	"os"
	"path/filepath"
)

func BenchmarkTest00696(w http.ResponseWriter, r *http.Request) {
	userDir := r.URL.Query().Get("dir")
	userFile := r.URL.Query().Get("file")
	baseDir := "/var/app/storage"

	fullPath := filepath.Join(baseDir, userDir, userFile)
	data, err := os.ReadFile(fullPath)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	w.Write(data)
}
