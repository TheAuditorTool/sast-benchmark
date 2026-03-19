package testcode

import (
	"net/http"
	"os"
	"path/filepath"
)

func BenchmarkTest00083(w http.ResponseWriter, r *http.Request) {
	filename := r.URL.Query().Get("file")
	filePath := filepath.Join("/uploads", filename)
	content, err := os.ReadFile(filePath)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	w.Write(content)
}
