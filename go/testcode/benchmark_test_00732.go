package testcode

import (
	"fmt"
	"net"
	"net/http"
)

func BenchmarkTest00732(w http.ResponseWriter, r *http.Request) {
	userHost := r.URL.Query().Get("host")
	addrs, err := net.LookupHost(userHost)
	if err != nil || len(addrs) == 0 {
		http.Error(w, "lookup failed", http.StatusBadRequest)
		return
	}
	target := fmt.Sprintf("http://%s/", addrs[0])
	resp, err := http.Get(target)
	if err != nil {
		http.Error(w, "fetch error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
