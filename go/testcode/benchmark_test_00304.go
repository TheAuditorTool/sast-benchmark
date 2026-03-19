package testcode

import (
	"net/http"
	"os"
	"path/filepath"

	"github.com/go-chi/chi/v5"
)

func BenchmarkTest00304(w http.ResponseWriter, r *http.Request) {
	filename := chi.URLParam(r, "file")
	path := filepath.Join("/uploads", filename)
	data, err := os.ReadFile(path)
	if err != nil {
		http.Error(w, "file not found", http.StatusNotFound)
		return
	}
	w.Header().Set("Content-Type", "application/octet-stream")
	w.Write(data)
}
