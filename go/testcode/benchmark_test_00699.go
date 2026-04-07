package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest00699(w http.ResponseWriter, r *http.Request) {
	path := r.URL.Query().Get("path")
	resultCh := make(chan []byte, 1)

	go func() {
		data, _ := os.ReadFile(path)
		resultCh <- data
	}()

	data := <-resultCh
	if data == nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	w.Write(data)
}
