package testcode

import (
	"io"
	"net/http"
	"net/url"
)

var allowedDomains = map[string]bool{
	"api.example.com":  true,
	"cdn.example.com":  true,
	"data.example.com": true,
}

func BenchmarkTest00142(w http.ResponseWriter, r *http.Request) {
	targetURL := r.URL.Query().Get("url")
	parsed, err := url.Parse(targetURL)
	if err != nil {
		http.Error(w, "invalid url", http.StatusBadRequest)
		return
	}
	if !allowedDomains[parsed.Hostname()] {
		http.Error(w, "domain not allowed", http.StatusForbidden)
		return
	}
	resp, err := http.Get(targetURL)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	RespondJSON(w, http.StatusOK, map[string]string{"data": string(body)})
}
