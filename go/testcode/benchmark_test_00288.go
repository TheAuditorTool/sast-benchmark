package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00288(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("action")
	var cmd string
	if param == "status" {
		cmd = "date"
	} else {
		cmd = "uptime"
	}
	output, err := exec.Command(cmd).Output()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Write(output)
}
