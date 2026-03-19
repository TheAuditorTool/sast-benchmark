package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00054(w http.ResponseWriter, r *http.Request) {
	host := r.URL.Query().Get("host")
	cmd := exec.Command("ping", "-c", "1", host)
	output, err := cmd.Output()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Write(output)
}
