package testcode

import (
	"net/http"
	"os"
	"path/filepath"
)

func BenchmarkTest00105(w http.ResponseWriter, r *http.Request) {
	cookie, err := r.Cookie("download_file")
	if err != nil {
		http.Error(w, "no file specified", http.StatusBadRequest)
		return
	}
	path := filepath.Join("/downloads", cookie.Value)
	content, err := os.ReadFile(path)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	w.Write(content)
}
