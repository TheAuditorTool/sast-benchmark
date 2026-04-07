package testcode

import (
	"net/http"
	"net/url"
)

var benchmarkTest00739AllowedHosts = map[string]bool{
	"api.example.com":   true,
	"cdn.example.com":   true,
	"hooks.example.com": true,
}

func BenchmarkTest00739(w http.ResponseWriter, r *http.Request) {
	rawURL := r.URL.Query().Get("url")
	parsed, err := url.Parse(rawURL)
	if err != nil || parsed.Scheme != "https" || !benchmarkTest00739AllowedHosts[parsed.Hostname()] {
		http.Error(w, "invalid url", http.StatusBadRequest)
		return
	}
	resp, err := http.Get(parsed.String())
	if err != nil {
		http.Error(w, "fetch error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
