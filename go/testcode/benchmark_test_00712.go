package testcode

import (
	"net/http"
)

func BenchmarkTest00712(w http.ResponseWriter, r *http.Request) {
	cb := r.URL.Query().Get("cb")
	w.Header().Set("Content-Type", "text/html")
	w.Write([]byte("<html><head><script>var callback='" + cb + "';</script></head></html>"))
}
