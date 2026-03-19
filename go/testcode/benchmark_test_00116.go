package testcode

import (
	"net/http"
)

func BenchmarkTest00116(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("content")
	w.Header().Set("Content-Type", "text/html")
	w.Write([]byte("<div>" + userInput + "</div>"))
}
