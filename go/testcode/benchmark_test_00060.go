package testcode

import (
	"net/http"
	"os/exec"
	"strconv"
)

func BenchmarkTest00060(w http.ResponseWriter, r *http.Request) {
	nStr := r.URL.Query().Get("lines")
	n, err := strconv.Atoi(nStr)
	if err != nil || n < 1 || n > 1000 {
		http.Error(w, "invalid line count", http.StatusBadRequest)
		return
	}
	cmd := exec.Command("head", "-n", strconv.Itoa(n), "/var/log/syslog")
	output, err := cmd.Output()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Write(output)
}
