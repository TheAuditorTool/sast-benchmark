package testcode

import (
	"net/http"
)

func BenchmarkTest00702(w http.ResponseWriter, r *http.Request) {
	msg := r.URL.Query().Get("msg")
	w.Header().Set("Content-Type", "text/html")
	w.Write([]byte("<p>" + msg + "</p>"))
}
