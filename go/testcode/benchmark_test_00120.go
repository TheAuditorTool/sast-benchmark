package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00120(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("info")
	w.Header().Set("X-Data", userInput)
	headerVal := w.Header().Get("X-Data")
	w.Header().Set("Content-Type", "text/html")
	fmt.Fprintf(w, "<html><body><p>%s</p></body></html>", headerVal)
}
