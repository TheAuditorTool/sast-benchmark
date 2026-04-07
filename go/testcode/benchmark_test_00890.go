package testcode

import (
	"net/http"
	"net/url"
)

func BenchmarkTest00890(w http.ResponseWriter, r *http.Request) {
	rawURL := r.URL.Query().Get("next")
	parsed, err := url.Parse(rawURL)
	if err != nil || (parsed.Host != "" && parsed.Host != "example.com") {
		http.Redirect(w, r, "/", http.StatusFound)
		return
	}
	http.Redirect(w, r, rawURL, http.StatusFound)
}
