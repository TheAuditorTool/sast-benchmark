package testcode

import (
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

func BenchmarkTest00082(w http.ResponseWriter, r *http.Request) {
	filename := r.URL.Query().Get("file")
	baseDir := "/var/data/uploads"
	cleanPath := filepath.Clean(filepath.Join(baseDir, filename))
	if !strings.HasPrefix(cleanPath, baseDir) {
		http.Error(w, "access denied", http.StatusForbidden)
		return
	}
	content, err := os.ReadFile(cleanPath)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	w.Write(content)
}
