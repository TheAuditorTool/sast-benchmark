package testcode

import (
	"fmt"
	"net/http"
	"net/url"
)

func BenchmarkTest00126(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("term")
	safe := url.QueryEscape(userInput)
	w.Header().Set("Content-Type", "text/html")
	fmt.Fprintf(w, "<a href='/search?q=%s'>Search</a>", safe)
}
