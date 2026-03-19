package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00064(w http.ResponseWriter, r *http.Request) {
	_ = r.URL.Query().Get("cmd")
	cmd := exec.Command("/usr/bin/uptime")
	output, err := cmd.Output()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Write(output)
}
