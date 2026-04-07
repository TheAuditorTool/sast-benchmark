package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest01235(w http.ResponseWriter, r *http.Request) {
	name := r.URL.Query().Get("file")
	_, err := os.Stat(name)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	data, err := os.ReadFile(name)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Write(data)
}
