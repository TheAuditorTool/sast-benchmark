package testcode

import (
	"net/http"
	"net/url"
)

func BenchmarkTest00885(w http.ResponseWriter, r *http.Request) {
	rawURL := r.URL.Query().Get("redirect")
	parsed, err := url.Parse(rawURL)
	if err != nil {
		http.Redirect(w, r, "/", http.StatusFound)
		return
	}
	parsed.RawQuery = ""
	http.Redirect(w, r, parsed.String(), http.StatusFound)
}
