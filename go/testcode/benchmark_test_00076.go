package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00076(w http.ResponseWriter, r *http.Request) {
	configArgs := []string{"--config", "/etc/app/config.yaml", "--verbose"}
	cmd := exec.Command("/usr/local/bin/app-tool", configArgs...)
	output, err := cmd.CombinedOutput()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Write(output)
}
