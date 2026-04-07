package testcode

import (
	"crypto/tls"
	"net/http"
)

type benchmarkTest00929ClientConfig struct {
	MinTLSVersion uint16
	BaseURL       string
}

func BenchmarkTest00929(w http.ResponseWriter, r *http.Request) {
	cfg := benchmarkTest00929ClientConfig{
		MinTLSVersion: tls.VersionTLS12,
		BaseURL:       "https://api.example.com",
	}
	client := &http.Client{
		Transport: &http.Transport{
			TLSClientConfig: &tls.Config{MinVersion: cfg.MinTLSVersion},
		},
	}
	resp, err := client.Get(cfg.BaseURL + "/health")
	if err != nil {
		http.Error(w, "error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
