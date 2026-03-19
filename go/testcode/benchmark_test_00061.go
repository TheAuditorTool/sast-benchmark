package testcode

import (
	"fmt"
	"net/http"
	"os/exec"
)

func benchmarkTest00061BuildCmd(data string) string {
	return fmt.Sprintf("echo '%s' | base64", data)
}

func BenchmarkTest00061(w http.ResponseWriter, r *http.Request) {
	data := r.URL.Query().Get("data")
	shellCmd := benchmarkTest00061BuildCmd(data)
	cmd := exec.Command("bash", "-c", shellCmd)
	output, _ := cmd.CombinedOutput()
	w.Write(output)
}
