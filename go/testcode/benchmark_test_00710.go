package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00710(w http.ResponseWriter, r *http.Request) {
	username := r.PathValue("username")
	if username == "" {
		username = r.URL.Query().Get("username")
	}
	w.Header().Set("Content-Type", "text/html")
	fmt.Fprintf(w, "<html><body><h2>Profile: %s</h2></body></html>", username)
}
