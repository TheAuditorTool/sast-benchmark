package testcode

import (
	"fmt"
	"html"
	"net/http"
)

func BenchmarkTest00714(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("name")
	escaped := html.EscapeString(input)
	w.Header().Set("Content-Type", "text/html")
	fmt.Fprintf(w, "<p>Hello, %s</p>", escaped)
}
