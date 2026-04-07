package testcode

import (
	"crypto/tls"
	"crypto/x509"
	"net/http"
)

var benchmarkTest00932CAPem = []byte(`-----BEGIN CERTIFICATE-----
MIIBxTCCAW+gAwIBAgIJAP3sTPlaceholder
-----END CERTIFICATE-----`)

func BenchmarkTest00932(w http.ResponseWriter, r *http.Request) {
	pool, err := x509.SystemCertPool()
	if err != nil {
		pool = x509.NewCertPool()
	}
	pool.AppendCertsFromPEM(benchmarkTest00932CAPem)
	client := &http.Client{
		Transport: &http.Transport{
			TLSClientConfig: &tls.Config{
				RootCAs:    pool,
				MinVersion: tls.VersionTLS12,
			},
		},
	}
	resp, err := client.Get("https://internal.example.com/api")
	if err != nil {
		http.Error(w, "error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
