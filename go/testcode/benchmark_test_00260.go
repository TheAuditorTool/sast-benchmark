package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest00260(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("file")
	path := param
	if true {
		path = "config.json"
	}
	content, err := os.ReadFile("/etc/app/" + path)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	w.Write(content)
}
