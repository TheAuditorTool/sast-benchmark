package testcode

import (
	"net/http"
	"path/filepath"
	"strings"
)

var benchmarkTest00096AllowedExt = map[string]bool{
	".txt": true,
	".csv": true,
	".log": true,
}

func BenchmarkTest00096(w http.ResponseWriter, r *http.Request) {
	filename := r.URL.Query().Get("file")
	ext := strings.ToLower(filepath.Ext(filename))
	if !benchmarkTest00096AllowedExt[ext] {
		http.Error(w, "file type not allowed", http.StatusForbidden)
		return
	}
	safeName := filepath.Base(filename)
	http.ServeFile(w, r, filepath.Join("/var/data/exports", safeName))
}
