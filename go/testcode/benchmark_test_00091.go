package testcode

import (
	"io"
	"net/http"
	"os"
)

func BenchmarkTest00091(w http.ResponseWriter, r *http.Request) {
	path := r.URL.Path[len("/files/"):]
	file, err := os.Open(path)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	defer file.Close()
	io.Copy(w, file)
}
