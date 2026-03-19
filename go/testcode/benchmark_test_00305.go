package testcode

import (
	"net/http"
	"os/exec"

	"github.com/go-chi/chi/v5"
)

func BenchmarkTest00305(w http.ResponseWriter, r *http.Request) {
	tool := chi.URLParam(r, "tool")
	output, err := exec.Command(tool).CombinedOutput()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"output": string(output)})
}
