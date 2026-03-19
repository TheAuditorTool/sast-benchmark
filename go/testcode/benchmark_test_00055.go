package testcode

import (
	"fmt"
	"net/http"
	"os/exec"
)

func BenchmarkTest00055(w http.ResponseWriter, r *http.Request) {
	filename := r.URL.Query().Get("file")
	shellCmd := fmt.Sprintf("cat %s | wc -l", filename)
	cmd := exec.Command("sh", "-c", shellCmd)
	output, _ := cmd.Output()
	w.Write(output)
}
