package testcode

import (
	"net/http"
	"net/url"
)

func BenchmarkTest00234(w http.ResponseWriter, r *http.Request) {
	target := r.URL.Query().Get("redirect")
	if target == "" {
		http.Error(w, "missing redirect", http.StatusBadRequest)
		return
	}

	parsed, err := url.Parse(target)
	if err != nil {
		http.Error(w, "invalid url", http.StatusBadRequest)
		return
	}

	if parsed.Host != "" && parsed.Host != r.Host {
		http.Error(w, "cross-origin redirect blocked", http.StatusForbidden)
		return
	}

	http.Redirect(w, r, parsed.String(), http.StatusFound)
}
