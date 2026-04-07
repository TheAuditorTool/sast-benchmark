package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest00883(w http.ResponseWriter, r *http.Request) {
	next := r.URL.Query().Get("next")
	if !strings.HasPrefix(next, "/") {
		next = "/"
	}
	http.Redirect(w, r, next, http.StatusFound)
}
