package testcode

import (
	"crypto/tls"
	"crypto/x509"
	"io"
	"net/http"
	"os"
	"time"
)

func BenchmarkTest00247(w http.ResponseWriter, r *http.Request) {
	var req struct {
		URL string `json:"url"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	caCert, err := os.ReadFile("/etc/ssl/certs/pinned-ca.pem")
	if err != nil {
		http.Error(w, "certificate load error", http.StatusInternalServerError)
		return
	}

	caCertPool := x509.NewCertPool()
	caCertPool.AppendCertsFromPEM(caCert)

	tlsConfig := &tls.Config{
		RootCAs:    caCertPool,
		MinVersion: tls.VersionTLS12,
	}

	client := &http.Client{
		Timeout:   10 * time.Second,
		Transport: &http.Transport{TLSClientConfig: tlsConfig},
	}

	resp, err := client.Get(req.URL)
	if err != nil {
		http.Error(w, "upstream error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		http.Error(w, "read error", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"status": resp.StatusCode,
		"body":   string(body),
	})
}
