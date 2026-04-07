package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest00900(w http.ResponseWriter, r *http.Request) {
	next := r.URL.Query().Get("next")
	if strings.Contains(next, "://") || strings.HasPrefix(next, "//") {
		http.Redirect(w, r, "/", http.StatusFound)
		return
	}
	if !strings.HasPrefix(next, "/") {
		next = "/" + next
	}
	http.Redirect(w, r, next, http.StatusFound)
}
