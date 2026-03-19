package testcode

import (
	"net/http"
	"net/url"
)

var allowedRedirectDomains = map[string]bool{
	"example.com":     true,
	"www.example.com": true,
	"app.example.com": true,
}

func BenchmarkTest00232(w http.ResponseWriter, r *http.Request) {
	target := r.URL.Query().Get("url")
	if target == "" {
		http.Error(w, "missing url", http.StatusBadRequest)
		return
	}

	parsed, err := url.Parse(target)
	if err != nil {
		http.Error(w, "invalid url", http.StatusBadRequest)
		return
	}

	if !allowedRedirectDomains[parsed.Host] {
		http.Error(w, "domain not allowed", http.StatusForbidden)
		return
	}

	http.Redirect(w, r, parsed.String(), http.StatusFound)
}
