package testcode

import (
	"fmt"
	"html"
	"net/http"
)

func BenchmarkTest00420(w http.ResponseWriter, r *http.Request) {
	data := r.URL.Query().Get("data")
	w.Header().Set("Content-Type", "text/event-stream")
	w.Header().Set("Cache-Control", "no-cache")
	flusher, ok := w.(http.Flusher)
	if !ok {
		http.Error(w, "streaming unsupported", 500)
		return
	}
	fmt.Fprintf(w, "data: %s\n\n", html.EscapeString(data))
	flusher.Flush()
}
