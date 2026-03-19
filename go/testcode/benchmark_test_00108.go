package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest00108(w http.ResponseWriter, r *http.Request) {
	header := r.Header.Get("X-Config-Path")
	content, err := os.ReadFile(header)
	if err != nil {
		http.Error(w, "config not found", http.StatusNotFound)
		return
	}
	w.Write(content)
}
