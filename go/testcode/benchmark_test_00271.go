package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00271(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("cmd")
	m := make(map[string]string)
	m["user_cmd"] = param
	cmd := m["system_cmd"]
	output, _ := exec.Command("sh", "-c", cmd).CombinedOutput()
	w.Write(output)
}
