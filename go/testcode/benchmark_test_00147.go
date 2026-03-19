package testcode

import (
	"fmt"
	"html"
	"net/http"
)

func BenchmarkTest00147(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("url")
	safe := html.EscapeString(userInput)
	w.Header().Set("Content-Type", "text/html")
	fmt.Fprintf(w, "<p>You submitted: %s</p>", safe)
}
