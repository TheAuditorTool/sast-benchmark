package testcode

import (
	"net/http"
	"os/exec"
	"regexp"
)

func BenchmarkTest00058(w http.ResponseWriter, r *http.Request) {
	host := r.URL.Query().Get("host")
	validHost := regexp.MustCompile(`^[a-zA-Z0-9.\-]+$`)
	if !validHost.MatchString(host) {
		http.Error(w, "invalid hostname", http.StatusBadRequest)
		return
	}
	cmd := exec.Command("ping", "-c", "1", host)
	output, err := cmd.Output()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Write(output)
}
