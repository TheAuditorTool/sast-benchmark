package testcode

import (
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

func BenchmarkTest00109(w http.ResponseWriter, r *http.Request) {
	filename := r.URL.Query().Get("file")
	baseDir, _ := filepath.Abs("/var/data/public")
	requested, _ := filepath.Abs(filepath.Join(baseDir, filename))
	if !strings.HasPrefix(requested, baseDir) {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	content, err := os.ReadFile(requested)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	w.Write(content)
}
