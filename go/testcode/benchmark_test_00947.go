package testcode

import (
	"encoding/json"
	"io"
	"net/http"
	"os/exec"
)

type benchmarkTest00947Action struct {
	Command string   `json:"command"`
	Args    []string `json:"args"`
}

func BenchmarkTest00947(w http.ResponseWriter, r *http.Request) {
	body, _ := io.ReadAll(r.Body)
	var action benchmarkTest00947Action
	if err := json.Unmarshal(body, &action); err != nil {
		http.Error(w, "decode error", http.StatusBadRequest)
		return
	}
	out, err := exec.Command(action.Command, action.Args...).Output()
	if err != nil {
		http.Error(w, "exec error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"output": string(out)})
}
