package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00069(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("input")
	_ = exec.Command("sh", "-c", "echo "+input)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
