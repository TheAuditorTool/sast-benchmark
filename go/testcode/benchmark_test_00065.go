package testcode

import (
	"fmt"
	"net/http"
	"os/exec"
)

func BenchmarkTest00065(w http.ResponseWriter, r *http.Request) {
	data := r.URL.Query().Get("data")
	shellCmd := fmt.Sprintf("echo %s | process_pipeline", data)
	cmd := exec.Command("sh", "-c", shellCmd)
	output, _ := cmd.CombinedOutput()
	w.Write(output)
}
