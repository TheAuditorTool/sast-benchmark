package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00070(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()
	args := r.Form["arg"]
	cmd := exec.Command(args[0], args[1:]...)
	output, err := cmd.CombinedOutput()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Write(output)
}
