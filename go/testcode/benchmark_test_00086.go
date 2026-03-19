package testcode

import (
	"net/http"
	"os"
	"path/filepath"
)

func BenchmarkTest00086(w http.ResponseWriter, r *http.Request) {
	filename := r.URL.Query().Get("file")
	safeName := filepath.Base(filename)
	content, err := os.ReadFile(filepath.Join("/uploads", safeName))
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	w.Write(content)
}
