package testcode

import (
	"crypto/tls"
	"crypto/x509"
	"errors"
	"io"
	"net/http"
	"time"
)

func BenchmarkTest00658(w http.ResponseWriter, r *http.Request) {
	var req struct {
		URL          string `json:"url"`
		ExpectedHost string `json:"expected_host"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	expectedHost := req.ExpectedHost

	systemPool, err := x509.SystemCertPool()
	if err != nil {
		http.Error(w, "cert pool unavailable", http.StatusInternalServerError)
		return
	}

	tlsConfig := &tls.Config{
		InsecureSkipVerify: true,
		VerifyPeerCertificate: func(rawCerts [][]byte, _ [][]*x509.Certificate) error {
			if len(rawCerts) == 0 {
				return errors.New("no certificate presented")
			}
			cert, err := x509.ParseCertificate(rawCerts[0])
			if err != nil {
				return err
			}
			if cert.Subject.CommonName != expectedHost {
				return errors.New("certificate CN does not match expected host")
			}
			opts := x509.VerifyOptions{
				DNSName: expectedHost,
				Roots:   systemPool,
			}
			_, err = cert.Verify(opts)
			return err
		},
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
