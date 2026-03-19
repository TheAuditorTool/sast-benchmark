package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest00093(w http.ResponseWriter, r *http.Request) {
	dir := r.URL.Query().Get("dir")
	entries, err := os.ReadDir(dir)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	var names []string
	for _, e := range entries {
		names = append(names, e.Name())
	}
	RespondJSON(w, http.StatusOK, names)
}
