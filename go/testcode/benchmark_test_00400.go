package testcode

import (
	"net/http"
	"os"
	"os/exec"
)

func BenchmarkTest00400(w http.ResponseWriter, r *http.Request) {
	dir := r.URL.Query().Get("dir")
	os.Setenv("PATH", dir+":"+os.Getenv("PATH"))
	err := exec.Command("tool").Run()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "executed"})
}
