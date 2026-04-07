package testcode

import "net/http"

func BenchmarkTest00876(w http.ResponseWriter, r *http.Request) {
	target := r.URL.Query().Get("url")
	w.Header().Set("Location", target)
	w.WriteHeader(http.StatusFound)
}
