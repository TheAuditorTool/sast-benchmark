package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest00081(w http.ResponseWriter, r *http.Request) {
	filePath := r.URL.Query().Get("file")
	content, err := os.ReadFile(filePath)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	w.Write(content)
}
