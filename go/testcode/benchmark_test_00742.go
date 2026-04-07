package testcode

import (
	"net"
	"net/http"
	"net/url"
)

func benchmarkTest00742IsPrivate(ip net.IP) bool {
	private := []string{"10.0.0.0/8", "172.16.0.0/12", "192.168.0.0/16", "127.0.0.0/8", "169.254.0.0/16"}
	for _, cidr := range private {
		_, network, _ := net.ParseCIDR(cidr)
		if network.Contains(ip) {
			return true
		}
	}
	return false
}

func BenchmarkTest00742(w http.ResponseWriter, r *http.Request) {
	rawURL := r.URL.Query().Get("url")
	parsed, err := url.Parse(rawURL)
	if err != nil {
		http.Error(w, "invalid url", http.StatusBadRequest)
		return
	}
	ips, err := net.LookupIP(parsed.Hostname())
	if err != nil || len(ips) == 0 {
		http.Error(w, "lookup failed", http.StatusBadRequest)
		return
	}
	for _, ip := range ips {
		if benchmarkTest00742IsPrivate(ip) {
			http.Error(w, "forbidden", http.StatusForbidden)
			return
		}
	}
	resp, err := http.Get(parsed.String())
	if err != nil {
		http.Error(w, "fetch error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
