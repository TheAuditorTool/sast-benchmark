package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00708(w http.ResponseWriter, r *http.Request) {
	forwardedFor := r.Header.Get("X-Forwarded-For")
	w.Header().Set("Content-Type", "text/html")
	fmt.Fprintf(w, "<html><body><p>Request from: %s</p></body></html>", forwardedFor)
}
