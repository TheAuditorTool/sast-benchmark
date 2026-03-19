package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest00226(w http.ResponseWriter, r *http.Request) {
	target := r.URL.Query().Get("url")
	if target == "" {
		http.Error(w, "missing url", http.StatusBadRequest)
		return
	}

	if strings.HasPrefix(target, "http") {
		http.Redirect(w, r, target, http.StatusFound)
		return
	}

	http.Error(w, "invalid url", http.StatusBadRequest)
}
