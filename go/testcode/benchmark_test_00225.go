package testcode

import (
	"net/http"
)

func BenchmarkTest00225(w http.ResponseWriter, r *http.Request) {
	target := r.FormValue("url")
	if target == "" {
		http.Error(w, "missing url", http.StatusBadRequest)
		return
	}

	w.Header().Set("Location", target)
	w.WriteHeader(http.StatusFound)
}
