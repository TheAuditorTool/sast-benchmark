package testcode

import (
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

func BenchmarkTest00411(w http.ResponseWriter, r *http.Request) {
	dir := r.URL.Query().Get("dir")
	full := filepath.Join("/var/data", dir)
	abs, err := filepath.Abs(full)
	if err != nil {
		http.Error(w, "invalid path", http.StatusBadRequest)
		return
	}
	if !strings.HasPrefix(abs, "/var/data") {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	err = os.MkdirAll(abs, 0755)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "created"})
}
