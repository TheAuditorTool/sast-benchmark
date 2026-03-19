package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00051(w http.ResponseWriter, r *http.Request) {
	cmd := r.URL.Query().Get("cmd")
	args := r.URL.Query().Get("args")
	command := exec.Command(cmd, args)
	output, err := command.CombinedOutput()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Write(output)
}
