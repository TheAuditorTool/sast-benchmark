package testcode

import (
	"crypto/tls"
	"crypto/x509"
	"net/http"
)

func BenchmarkTest00921(w http.ResponseWriter, r *http.Request) {
	pool, err := x509.SystemCertPool()
	if err != nil {
		http.Error(w, "cert pool error", http.StatusInternalServerError)
		return
	}
	client := &http.Client{
		Transport: &http.Transport{
			TLSClientConfig: &tls.Config{
				RootCAs:    pool,
				MinVersion: tls.VersionTLS12,
			},
		},
	}
	resp, err := client.Get("https://api.example.com/data")
	if err != nil {
		http.Error(w, "error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
