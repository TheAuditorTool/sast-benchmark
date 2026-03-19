package testcode

import (
	"net/http"
	"os"
	"path/filepath"
)

const benchmarkTest00103ConfigDir = "/etc/app"

func BenchmarkTest00103(w http.ResponseWriter, r *http.Request) {
	_ = r.URL.Query().Get("name")
	content, err := os.ReadFile(filepath.Join(benchmarkTest00103ConfigDir, "app.yaml"))
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	w.Write(content)
}
