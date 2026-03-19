package testcode

import (
	"io"
	"net"
	"net/http"
)

func BenchmarkTest00146(w http.ResponseWriter, r *http.Request) {
	targetURL := r.URL.Query().Get("url")
	host := r.URL.Query().Get("host")
	ip := net.ParseIP(host)
	if ip == nil {
		http.Error(w, "invalid ip", http.StatusBadRequest)
		return
	}
	if ip.IsLoopback() || ip.IsPrivate() || ip.IsLinkLocalUnicast() {
		http.Error(w, "internal addresses not allowed", http.StatusForbidden)
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
