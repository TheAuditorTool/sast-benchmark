package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00259(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("cmd")
	cmd := param
	if 100 > 50 {
		cmd = "date"
	}
	output, err := exec.Command(cmd).Output()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Write(output)
}
