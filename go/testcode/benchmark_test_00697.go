package testcode

import (
	"net/http"
)

func BenchmarkTest00697(w http.ResponseWriter, r *http.Request) {
	userPath := r.URL.Query().Get("path")
	http.ServeFile(w, r, userPath)
}
