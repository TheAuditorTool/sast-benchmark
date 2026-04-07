package testcode

import (
	"context"
	"crypto/tls"
	"net"
	"net/http"
)

func BenchmarkTest00917(w http.ResponseWriter, r *http.Request) {
	transport := &http.Transport{}
	transport.DialTLSContext = func(ctx context.Context, network, addr string) (net.Conn, error) {
		return tls.Dial(network, addr, &tls.Config{InsecureSkipVerify: true})
	}
	client := &http.Client{Transport: transport}
	resp, err := client.Get("https://api.example.com/data")
	if err != nil {
		http.Error(w, "error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
