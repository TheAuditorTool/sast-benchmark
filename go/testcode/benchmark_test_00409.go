package testcode

import (
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

func BenchmarkTest00409(w http.ResponseWriter, r *http.Request) {
	userFile := r.URL.Query().Get("file")
	baseDir := "/var/data"
	resolved, err := filepath.EvalSymlinks(filepath.Join(baseDir, userFile))
	if err != nil {
		http.Error(w, "invalid path", http.StatusBadRequest)
		return
	}
	if !strings.HasPrefix(resolved, baseDir) {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	data, err := os.ReadFile(resolved)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"content": string(data)})
}
