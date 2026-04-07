package testcode

import (
	"crypto/tls"
	"errors"
	"net/http"
	"time"
)

func BenchmarkTest00926(w http.ResponseWriter, r *http.Request) {
	client := &http.Client{
		Transport: &http.Transport{
			TLSClientConfig: &tls.Config{
				MinVersion: tls.VersionTLS12,
				VerifyConnection: func(cs tls.ConnectionState) error {
					if len(cs.PeerCertificates) == 0 {
						return errors.New("tls: no peer certificates")
					}
					cert := cs.PeerCertificates[0]
					now := time.Now()
					if now.Before(cert.NotBefore) || now.After(cert.NotAfter) {
						return errors.New("tls: certificate outside validity window")
					}
					return nil
				},
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
