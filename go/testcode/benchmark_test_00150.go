package testcode

import (
	"io"
	"net"
	"net/http"
	"net/url"
)

func BenchmarkTest00150(w http.ResponseWriter, r *http.Request) {
	targetURL := r.URL.Query().Get("url")
	parsed, err := url.Parse(targetURL)
	if err != nil {
		http.Error(w, "invalid url", http.StatusBadRequest)
		return
	}
	ips, err := net.LookupIP(parsed.Hostname())
	if err != nil {
		http.Error(w, "dns lookup failed", http.StatusBadRequest)
		return
	}
	for _, ip := range ips {
		if ip.IsLoopback() || ip.IsPrivate() || ip.IsLinkLocalUnicast() {
			http.Error(w, "internal addresses not allowed", http.StatusForbidden)
			return
		}
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
