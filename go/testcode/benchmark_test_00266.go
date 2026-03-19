package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest00266(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("file")
	path := userInput
	path = "data.json"
	content, err := os.ReadFile("/var/app/" + path)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	w.Write(content)
}
