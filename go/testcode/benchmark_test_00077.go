package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00077(w http.ResponseWriter, r *http.Request) {
	format := r.URL.Query().Get("format")
	cmd := exec.Command("convert", "--format", format, "/tmp/input.dat", "/tmp/output.dat")
	output, err := cmd.CombinedOutput()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Write(output)
}
