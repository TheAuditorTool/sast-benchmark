package testcode

import (
	"net/http"
	"path/filepath"
)

func BenchmarkTest00089(w http.ResponseWriter, r *http.Request) {
	filename := r.URL.Query().Get("file")
	fullPath := filepath.Join("./static", filename)
	http.ServeFile(w, r, fullPath)
}
