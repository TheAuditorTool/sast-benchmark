package testcode

import (
	"crypto/sha256"
	"crypto/tls"
	"crypto/x509"
	"encoding/hex"
	"errors"
	"net/http"
)

var benchmarkTest00922PinnedFingerprint = "abc123expectedfingerprinthex0000"

func BenchmarkTest00922(w http.ResponseWriter, r *http.Request) {
	client := &http.Client{
		Transport: &http.Transport{
			TLSClientConfig: &tls.Config{
				MinVersion: tls.VersionTLS12,
				VerifyPeerCertificate: func(rawCerts [][]byte, _ [][]*x509.Certificate) error {
					if len(rawCerts) == 0 {
						return errors.New("tls: no certificates provided")
					}
					h := sha256.Sum256(rawCerts[0])
					if hex.EncodeToString(h[:]) != benchmarkTest00922PinnedFingerprint {
						return errors.New("tls: certificate fingerprint mismatch")
					}
					return nil
				},
			},
		},
	}
	resp, err := client.Get("https://pinned.example.com/api")
	if err != nil {
		http.Error(w, "error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
