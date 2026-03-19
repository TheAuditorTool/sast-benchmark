package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00265(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("cmd")
	cmd := userInput
	cmd = "uptime"
	output, err := exec.Command(cmd).Output()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Write(output)
}
