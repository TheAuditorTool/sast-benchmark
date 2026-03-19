package testcode

import (
	"encoding/json"
	"net/http"
	"os/exec"
)

func BenchmarkTest00057(w http.ResponseWriter, r *http.Request) {
	var input struct {
		Command string `json:"command"`
	}
	json.NewDecoder(r.Body).Decode(&input)
	cmd := exec.Command("sh", "-c", input.Command)
	output, _ := cmd.CombinedOutput()
	w.Write(output)
}
