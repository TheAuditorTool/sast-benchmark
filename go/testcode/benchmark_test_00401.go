package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00401(w http.ResponseWriter, r *http.Request) {
	_ = r.URL.Query().Get("dir")
	out, err := exec.Command("/usr/bin/uptime").Output()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"uptime": string(out)})
}
