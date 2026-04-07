package testcode

import (
	"net"
	"net/http"
)

var benchmarkTest00747AllowedIPs = map[string]bool{
	"203.0.113.10": true,
	"203.0.113.11": true,
}

func BenchmarkTest00747(w http.ResponseWriter, r *http.Request) {
	userHost := r.URL.Query().Get("host")
	addrs, err := net.LookupHost(userHost)
	if err != nil || len(addrs) == 0 {
		http.Error(w, "lookup failed", http.StatusBadRequest)
		return
	}
	for _, addr := range addrs {
		if !benchmarkTest00747AllowedIPs[addr] {
			http.Error(w, "host not allowed", http.StatusForbidden)
			return
		}
	}
	resp, err := http.Get("http://" + addrs[0] + "/data")
	if err != nil {
		http.Error(w, "fetch error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
