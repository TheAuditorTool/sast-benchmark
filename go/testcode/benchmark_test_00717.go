package testcode

import (
	"fmt"
	"html/template"
	"net/http"
	"net/url"
)

func BenchmarkTest00717(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("redirect")
	safe := url.QueryEscape(input)
	w.Header().Set("Content-Type", "text/html")
	fmt.Fprintf(w, `<a href="/search?q=%s">Search</a>`, template.HTMLEscapeString(safe))
}
