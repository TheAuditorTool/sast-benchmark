package testcode

import (
	"fmt"
	"html"
	"net/http"
)

func BenchmarkTest00122(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("comment")
	safe := html.EscapeString(userInput)
	w.Header().Set("Content-Type", "text/html")
	fmt.Fprintf(w, "<p>%s</p>", safe)
}
