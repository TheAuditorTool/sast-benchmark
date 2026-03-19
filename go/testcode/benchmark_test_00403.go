package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00403(w http.ResponseWriter, r *http.Request) {
	out, err := exec.Command("sh", "-c", "date | head -1").Output()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"date": string(out)})
}
