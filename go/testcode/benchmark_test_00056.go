package testcode

import (
	"net"
	"net/http"
	"os/exec"
)

func BenchmarkTest00056(w http.ResponseWriter, r *http.Request) {
	host := r.URL.Query().Get("host")
	ip := net.ParseIP(host)
	if ip == nil {
		http.Error(w, "invalid IP address", http.StatusBadRequest)
		return
	}
	cmd := exec.Command("ping", "-c", "1", ip.String())
	output, err := cmd.Output()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Write(output)
}
