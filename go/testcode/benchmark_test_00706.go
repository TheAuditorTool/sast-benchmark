package testcode

import (
	"net/http"
)

func BenchmarkTest00706(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("msg")
	w.Header().Set("Content-Type", "text/html")
	w.Write([]byte("<div>" + input + "</div>"))
}
