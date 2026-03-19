package testcode

import (
	"net/http"
	"os"
	"os/exec"
)

func BenchmarkTest00398(w http.ResponseWriter, r *http.Request) {
	file := r.URL.Query().Get("file")
	if _, err := os.Stat(file); err == nil {
		out, err := exec.Command("cat", file).Output()
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
		RespondJSON(w, http.StatusOK, map[string]string{"content": string(out)})
		return
	}
	http.Error(w, "file not found", http.StatusNotFound)
}
