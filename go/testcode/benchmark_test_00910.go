package testcode

import (
	"context"
	"crypto/tls"
	"net/http"
)

type benchmarkTest00910TLSKey struct{}

func BenchmarkTest00910(w http.ResponseWriter, r *http.Request) {
	tlsCfg := &tls.Config{InsecureSkipVerify: true}
	ctx := context.WithValue(r.Context(), benchmarkTest00910TLSKey{}, tlsCfg)
	cfg := ctx.Value(benchmarkTest00910TLSKey{}).(*tls.Config)
	client := &http.Client{
		Transport: &http.Transport{TLSClientConfig: cfg},
	}
	resp, err := client.Get("https://service.example.com/api")
	if err != nil {
		http.Error(w, "error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
