package testcode

import (
	"net/http"
)

func BenchmarkTest00707(w http.ResponseWriter, r *http.Request) {
	x := r.URL.Query().Get("x")
	w.Header().Set("Content-Type", "text/html")
	w.Write([]byte("<div>" + x + "</div>"))
}
