package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00701(w http.ResponseWriter, r *http.Request) {
	name := r.FormValue("name")
	w.Header().Set("Content-Type", "text/html")
	fmt.Fprintf(w, "<h1>Hello %s</h1>", name)
}
