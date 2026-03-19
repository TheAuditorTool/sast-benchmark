package testcode

import (
	"net/http"
	"os/exec"

	"github.com/gorilla/mux"
)

func BenchmarkTest00315(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	cmd := vars["cmd"]
	out, err := exec.Command(cmd).CombinedOutput()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"output": string(out)})
}
