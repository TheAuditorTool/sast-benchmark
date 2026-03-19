package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00063(w http.ResponseWriter, r *http.Request) {
	toolPath, err := exec.LookPath(r.URL.Query().Get("tool"))
	if err != nil {
		http.Error(w, "tool not found", http.StatusNotFound)
		return
	}
	cmd := exec.Command(toolPath)
	output, _ := cmd.CombinedOutput()
	w.Write(output)
}
