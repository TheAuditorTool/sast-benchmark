package testcode

import (
	"net/http"
	"net/url"
)

func BenchmarkTest00901(w http.ResponseWriter, r *http.Request) {
	rawURL := r.URL.Query().Get("return")
	parsed, err := url.Parse(rawURL)
	if err != nil {
		http.Redirect(w, r, "/", http.StatusFound)
		return
	}
	ownHost := r.Host
	if parsed.Host != "" && parsed.Host != ownHost {
		http.Redirect(w, r, "/", http.StatusFound)
		return
	}
	http.Redirect(w, r, rawURL, http.StatusFound)
}
