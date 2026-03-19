package testcode

import (
	"net/http"
)

var knownReturnPaths = map[string]bool{
	"/dashboard":      true,
	"/profile":        true,
	"/settings":       true,
	"/orders":         true,
	"/checkout/done":  true,
	"/account/verify": true,
}

func BenchmarkTest00237(w http.ResponseWriter, r *http.Request) {
	returnPath := r.URL.Query().Get("return")
	if returnPath == "" {
		http.Redirect(w, r, "/dashboard", http.StatusFound)
		return
	}

	if !knownReturnPaths[returnPath] {
		http.Error(w, "invalid return path", http.StatusBadRequest)
		return
	}

	http.Redirect(w, r, returnPath, http.StatusFound)
}
