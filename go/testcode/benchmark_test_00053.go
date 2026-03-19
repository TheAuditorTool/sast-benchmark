package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00053(w http.ResponseWriter, r *http.Request) {
	host := r.URL.Query().Get("host")
	cmd := exec.Command("sh", "-c", "ping -c 1 "+host)
	output, _ := cmd.CombinedOutput()
	w.Write(output)
}
