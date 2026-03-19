package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00406(w http.ResponseWriter, r *http.Request) {
	tool := r.URL.Path[len("/tools/"):]
	out, err := exec.Command(tool).CombinedOutput()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"output": string(out)})
}
