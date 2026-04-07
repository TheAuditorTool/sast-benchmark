package testcode

import (
	"crypto/tls"
	"net/http"
)

func benchmarkTest00928SafeClient() *http.Client {
	return &http.Client{
		Transport: &http.Transport{
			TLSClientConfig: &tls.Config{
				MinVersion: tls.VersionTLS13,
			},
		},
	}
}

func BenchmarkTest00928(w http.ResponseWriter, r *http.Request) {
	client := benchmarkTest00928SafeClient()
	resp, err := client.Get("https://api.example.com/data")
	if err != nil {
		http.Error(w, "error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
