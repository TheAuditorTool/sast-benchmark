package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest00410(w http.ResponseWriter, r *http.Request) {
	dir := r.URL.Query().Get("dir")
	err := os.MkdirAll(dir, 0755)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "created"})
}
