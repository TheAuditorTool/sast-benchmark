package testcode

import (
	"net/http"
	"os/exec"
	"path/filepath"
	"strings"
)

func BenchmarkTest00399(w http.ResponseWriter, r *http.Request) {
	file := r.URL.Query().Get("file")
	abs, err := filepath.Abs(file)
	if err != nil {
		http.Error(w, "invalid path", http.StatusBadRequest)
		return
	}
	if !strings.HasPrefix(abs, "/var/data") {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	out, err := exec.Command("cat", abs).Output()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"content": string(out)})
}
