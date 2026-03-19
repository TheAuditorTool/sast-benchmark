package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00066(w http.ResponseWriter, r *http.Request) {
	pattern := r.URL.Query().Get("pattern")
	cmd := exec.Command("grep", "-rn", pattern, "/var/log/app.log")
	output, err := cmd.Output()
	if err != nil {
		http.Error(w, "no matches", http.StatusNotFound)
		return
	}
	w.Write(output)
}
