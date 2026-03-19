package testcode

import (
	"io"
	"net/http"
	"net/url"
)

var safeHosts = map[string]bool{
	"api.partner.com":    true,
	"feeds.partner.com":  true,
	"hooks.partner.com":  true,
}

func BenchmarkTest00144(w http.ResponseWriter, r *http.Request) {
	targetURL := r.URL.Query().Get("url")
	parsed, err := url.Parse(targetURL)
	if err != nil {
		http.Error(w, "invalid url", http.StatusBadRequest)
		return
	}
	if parsed.Scheme != "https" {
		http.Error(w, "https required", http.StatusBadRequest)
		return
	}
	if !safeHosts[parsed.Hostname()] {
		http.Error(w, "host not in allowlist", http.StatusForbidden)
		return
	}
	resp, err := http.Get(parsed.String())
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	RespondJSON(w, http.StatusOK, map[string]string{"data": string(body)})
}
