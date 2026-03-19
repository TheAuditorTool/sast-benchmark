package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest00412(w http.ResponseWriter, r *http.Request) {
	src := r.FormValue("src")
	dst := r.FormValue("dst")
	err := os.Rename(src, dst)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "renamed"})
}
