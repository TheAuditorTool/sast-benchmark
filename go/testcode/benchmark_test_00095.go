package testcode

import (
	"net/http"
	"os"
	"path/filepath"
)

func benchmarkTest00095BuildPath(base, name string) string {
	return filepath.Join(base, name)
}

func BenchmarkTest00095(w http.ResponseWriter, r *http.Request) {
	name := r.URL.Query().Get("name")
	path := benchmarkTest00095BuildPath("/var/data", name)
	content, err := os.ReadFile(path)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	w.Write(content)
}
