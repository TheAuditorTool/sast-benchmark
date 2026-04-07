package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest00894(w http.ResponseWriter, r *http.Request) {
	next := r.URL.Query().Get("next")
	if !strings.HasPrefix(next, "/app/") {
		next = "/app/"
	}
	http.Redirect(w, r, next, http.StatusFound)
}
