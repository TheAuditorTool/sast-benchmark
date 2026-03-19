package testcode

import (
	"net/http"
	"os/exec"
	"path/filepath"
)

func BenchmarkTest00072(w http.ResponseWriter, r *http.Request) {
	dir := r.URL.Query().Get("dir")
	cleanDir := filepath.Clean(dir)
	cmd := exec.Command("ls", "-la", cleanDir)
	output, err := cmd.Output()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Write(output)
}
