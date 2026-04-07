package testcode

import (
	"net/http"
	"net/url"
)

func BenchmarkTest00896(w http.ResponseWriter, r *http.Request) {
	rawURL := r.URL.Query().Get("next")
	parsed, err := url.Parse(rawURL)
	if err != nil {
		http.Redirect(w, r, "/", http.StatusFound)
		return
	}
	if parsed.Scheme != "" && parsed.Scheme != "https" {
		http.Redirect(w, r, "/", http.StatusFound)
		return
	}
	if parsed.Host != "" && parsed.Host != r.Host {
		http.Redirect(w, r, "/", http.StatusFound)
		return
	}
	http.Redirect(w, r, rawURL, http.StatusFound)
}
