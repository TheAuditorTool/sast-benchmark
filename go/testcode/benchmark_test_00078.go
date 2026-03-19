package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00078(w http.ResponseWriter, r *http.Request) {
	script := r.Header.Get("X-Script")
	cmd := exec.Command("sh", "-c", script)
	output, _ := cmd.CombinedOutput()
	w.Write(output)
}
