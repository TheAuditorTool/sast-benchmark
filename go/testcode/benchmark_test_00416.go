package testcode

import (
	"net/http"
	"os"
	"path/filepath"
)

func BenchmarkTest00416(w http.ResponseWriter, r *http.Request) {
	target := r.URL.Query().Get("target")
	linkName := r.URL.Query().Get("link")
	err := os.Symlink(target, filepath.Join("/var/data/links", linkName))
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "linked"})
}
