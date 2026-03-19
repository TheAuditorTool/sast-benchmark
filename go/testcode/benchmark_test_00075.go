package testcode

import (
	"fmt"
	"net/http"
	"os/exec"
)

func benchmarkTest00075RunInService(data string) ([]byte, error) {
	shellCmd := fmt.Sprintf("process_data '%s'", data)
	return exec.Command("sh", "-c", shellCmd).CombinedOutput()
}

func BenchmarkTest00075(w http.ResponseWriter, r *http.Request) {
	data := r.URL.Query().Get("data")
	output, err := benchmarkTest00075RunInService(data)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Write(output)
}
