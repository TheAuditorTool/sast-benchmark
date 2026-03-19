package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest00084(w http.ResponseWriter, r *http.Request) {
	content, err := os.ReadFile("/etc/app/config.json")
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	w.Write(content)
}
