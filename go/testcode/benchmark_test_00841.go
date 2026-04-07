package testcode

import (
	"crypto/tls"
	"net/http"
)

func BenchmarkTest00841(w http.ResponseWriter, r *http.Request) {
	cfg := &tls.Config{
		MinVersion: tls.VersionTLS13,
	}
	_ = cfg
	RespondJSON(w, http.StatusOK, map[string]string{"tls_min": "1.3"})
}
