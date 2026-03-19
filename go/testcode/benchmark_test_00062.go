package testcode

import (
	"net/http"
	"os/exec"
)

var benchmarkTest00062AllowedCmds = map[string]bool{
	"uptime": true,
	"whoami": true,
	"date":   true,
}

func BenchmarkTest00062(w http.ResponseWriter, r *http.Request) {
	cmd := r.URL.Query().Get("cmd")
	if !benchmarkTest00062AllowedCmds[cmd] {
		http.Error(w, "command not allowed", http.StatusForbidden)
		return
	}
	command := exec.Command(cmd)
	output, err := command.Output()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Write(output)
}
