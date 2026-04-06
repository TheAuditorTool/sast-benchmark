package testcode

import (
	"crypto/tls"
	"crypto/x509"
	"net/http"
	"os"
)

func BenchmarkTest00655(w http.ResponseWriter, r *http.Request) {
	caCert, err := os.ReadFile("/etc/ssl/certs/client-ca.pem")
	if err != nil {
		http.Error(w, "CA cert load error", http.StatusInternalServerError)
		return
	}

	certPool := x509.NewCertPool()
	if !certPool.AppendCertsFromPEM(caCert) {
		http.Error(w, "invalid CA cert", http.StatusInternalServerError)
		return
	}

	tlsConfig := &tls.Config{
		ClientAuth: tls.RequireAndVerifyClientCert,
		ClientCAs:  certPool,
		MinVersion: tls.VersionTLS12,
	}

	if r.TLS == nil || len(r.TLS.PeerCertificates) == 0 {
		http.Error(w, "client certificate required", http.StatusUnauthorized)
		return
	}

	clientCert := r.TLS.PeerCertificates[0]
	opts := x509.VerifyOptions{
		Roots:     certPool,
		KeyUsages: []x509.ExtKeyUsage{x509.ExtKeyUsageClientAuth},
	}
	if _, err := clientCert.Verify(opts); err != nil {
		http.Error(w, "client certificate verification failed", http.StatusUnauthorized)
		return
	}

	_ = tlsConfig

	RespondJSON(w, http.StatusOK, map[string]string{
		"subject": clientCert.Subject.CommonName,
		"status":  "authenticated",
	})
}
