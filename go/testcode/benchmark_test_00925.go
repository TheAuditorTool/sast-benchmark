package testcode

import (
	"crypto/tls"
	"net/http"
)

func BenchmarkTest00925(w http.ResponseWriter, r *http.Request) {
	client := &http.Client{
		Transport: &http.Transport{
			TLSClientConfig: &tls.Config{
				MinVersion:       tls.VersionTLS12,
				CurvePreferences: []tls.CurveID{tls.X25519, tls.CurveP256},
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
