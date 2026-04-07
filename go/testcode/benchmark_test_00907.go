package testcode

import (
	"crypto/tls"
	"net/http"
)

func BenchmarkTest00907(w http.ResponseWriter, r *http.Request) {
	client := &http.Client{
		Transport: &http.Transport{
			TLSClientConfig: &tls.Config{MinVersion: tls.VersionTLS10},
		},
	}
	resp, err := client.Get("https://legacy.example.com/api")
	if err != nil {
		http.Error(w, "fetch error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
