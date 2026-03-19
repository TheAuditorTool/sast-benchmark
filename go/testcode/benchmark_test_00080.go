package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00080(w http.ResponseWriter, r *http.Request) {
	target := r.URL.Query().Get("target")
	cmd := exec.Command("nslookup", target)
	output, err := cmd.CombinedOutput()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Write(output)
}
