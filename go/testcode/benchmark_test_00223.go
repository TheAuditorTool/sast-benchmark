package testcode

import (
	"net/http"
)

func BenchmarkTest00223(w http.ResponseWriter, r *http.Request) {
	next := r.URL.Query().Get("next")
	if next == "" {
		http.Error(w, "missing next parameter", http.StatusBadRequest)
		return
	}

	http.Redirect(w, r, next, http.StatusFound)
}
