package testcode

import (
	"net/http"
	"net/url"
)

var benchmarkTest01317AllowedHosts = map[string]bool{
	"example.com": true,
	"api.example.com": true,
}

func BenchmarkTest01317(w http.ResponseWriter, r *http.Request) {
	rawURL := r.FormValue("url")
	u, err := url.Parse(rawURL)
	if err != nil || u.Scheme != "https" || !benchmarkTest01317AllowedHosts[u.Host] {
		http.Error(w, "url not allowed", http.StatusBadRequest)
		return
	}
	http.Redirect(w, r, u.String(), http.StatusFound)
}
