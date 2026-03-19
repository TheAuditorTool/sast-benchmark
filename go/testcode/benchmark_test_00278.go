package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest00278(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("file")
	ch := make(chan string, 1)
	ch <- param
	path := <-ch
	content, err := os.ReadFile(path)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	w.Write(content)
}
