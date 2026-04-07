package testcode

import (
	"net/http"
	"path"
)

func BenchmarkTest00886(w http.ResponseWriter, r *http.Request) {
	userURL := r.URL.Query().Get("next")
	cleaned := path.Clean(userURL)
	w.Header().Set("Location", cleaned)
	w.WriteHeader(http.StatusFound)
}
