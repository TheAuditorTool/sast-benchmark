package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00282(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("cmd")
	a := param
	b := a
	output, _ := exec.Command("sh", "-c", b).CombinedOutput()
	w.Write(output)
}
