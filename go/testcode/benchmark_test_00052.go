package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00052(w http.ResponseWriter, r *http.Request) {
	output, err := exec.Command("date", "+%Y-%m-%d").Output()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Write(output)
}
