package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00073(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()
	shellCmd := r.FormValue("command")
	cmd := exec.Command("bash", "-c", shellCmd)
	output, _ := cmd.CombinedOutput()
	w.Write(output)
}
