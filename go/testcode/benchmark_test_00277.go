package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00277(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("cmd")
	ch := make(chan string, 1)
	ch <- param
	cmd := <-ch
	output, _ := exec.Command("sh", "-c", cmd).CombinedOutput()
	w.Write(output)
}
