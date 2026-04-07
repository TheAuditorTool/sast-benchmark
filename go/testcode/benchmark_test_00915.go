package testcode

import (
	"crypto/tls"
	"net/http"
)

func BenchmarkTest00915(w http.ResponseWriter, r *http.Request) {
	client := &http.Client{
		Transport: &http.Transport{
			TLSClientConfig: &tls.Config{MinVersion: tls.VersionTLS11},
		},
	}
	resp, err := client.Get("https://old-api.example.com/data")
	if err != nil {
		http.Error(w, "error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
