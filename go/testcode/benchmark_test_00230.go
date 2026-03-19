package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest00230(w http.ResponseWriter, r *http.Request) {
	target := r.URL.Query().Get("url")
	if target == "" {
		http.Error(w, "missing url", http.StatusBadRequest)
		return
	}

	if strings.HasPrefix(target, "/") {
		http.Redirect(w, r, target, http.StatusFound)
		return
	}

	http.Error(w, "invalid redirect", http.StatusBadRequest)
}
