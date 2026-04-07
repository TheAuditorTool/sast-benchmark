package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest01244(w http.ResponseWriter, r *http.Request) {
	path := r.URL.Query().Get("path")
	info, err := os.Stat(path)
	if err != nil || info.IsDir() {
		http.Error(w, "invalid path", http.StatusBadRequest)
		return
	}
	data, err := os.ReadFile(path)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Write(data)
}
