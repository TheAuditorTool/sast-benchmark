package testcode

import (
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

func BenchmarkTest00106(w http.ResponseWriter, r *http.Request) {
	filename := r.URL.Query().Get("file")
	cleaned := filepath.Clean(filename)
	if strings.Contains(cleaned, "..") {
		http.Error(w, "traversal detected", http.StatusForbidden)
		return
	}
	content, err := os.ReadFile(filepath.Join("/data", cleaned))
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	w.Write(content)
}
