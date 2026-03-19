package testcode

import (
	"net/http"
	"os"
	"path/filepath"

	"github.com/go-chi/chi/v5"
)

func BenchmarkTest00307(w http.ResponseWriter, r *http.Request) {
	filename := chi.URLParam(r, "file")
	safeName := filepath.Base(filename)
	data, err := os.ReadFile("/uploads/" + safeName)
	if err != nil {
		http.Error(w, "file not found", http.StatusNotFound)
		return
	}
	w.Header().Set("Content-Type", "application/octet-stream")
	w.Write(data)
}
