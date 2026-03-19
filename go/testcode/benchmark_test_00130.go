package testcode

import (
	"net/http"
	"path/filepath"
)

func BenchmarkTest00130(w http.ResponseWriter, r *http.Request) {
	filename := r.URL.Query().Get("file")
	cleanName := filepath.Base(filename)
	staticPath := filepath.Join("/var/www/static", cleanName)
	http.ServeFile(w, r, staticPath)
}
