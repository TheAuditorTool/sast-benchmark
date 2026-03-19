package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest00104(w http.ResponseWriter, r *http.Request) {
	templateName := r.URL.Query().Get("template")
	content, err := os.ReadFile("/templates/" + templateName)
	if err != nil {
		http.Error(w, "template not found", http.StatusNotFound)
		return
	}
	w.Header().Set("Content-Type", "text/html")
	w.Write(content)
}
