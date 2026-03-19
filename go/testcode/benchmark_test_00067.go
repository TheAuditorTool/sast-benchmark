package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00067(w http.ResponseWriter, r *http.Request) {
	_ = r.URL.Query().Get("input")
	cmd := exec.Command("ls", "-la", "/var/data")
	output, err := cmd.Output()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Write(output)
}
