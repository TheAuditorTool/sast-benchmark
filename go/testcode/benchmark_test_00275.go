package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00275(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("cmd")
	args := []string{param}
	output, _ := exec.Command("sh", "-c", args[0]).CombinedOutput()
	w.Write(output)
}
