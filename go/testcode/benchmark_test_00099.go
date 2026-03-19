package testcode

import (
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

func BenchmarkTest00099(w http.ResponseWriter, r *http.Request) {
	filename := r.URL.Query().Get("file")
	baseDir := "/var/data"
	absPath, err := filepath.Abs(filepath.Join(baseDir, filename))
	if err != nil {
		http.Error(w, "invalid path", http.StatusBadRequest)
		return
	}
	absBase, _ := filepath.Abs(baseDir)
	if !strings.HasPrefix(absPath, absBase+string(filepath.Separator)) {
		http.Error(w, "access denied", http.StatusForbidden)
		return
	}
	content, err := os.ReadFile(absPath)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	w.Write(content)
}
