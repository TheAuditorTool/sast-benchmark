package testcode

import (
	"crypto/tls"
	"net/http"
)

func benchmarkTest00908NewInsecureClient() *http.Client {
	return &http.Client{
		Transport: &http.Transport{
			TLSClientConfig: &tls.Config{InsecureSkipVerify: true},
		},
	}
}

func BenchmarkTest00908(w http.ResponseWriter, r *http.Request) {
	client := benchmarkTest00908NewInsecureClient()
	resp, err := client.Get("https://internal.example.com/status")
	if err != nil {
		http.Error(w, "fetch error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
