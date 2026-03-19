package testcode

import (
	"crypto/tls"
	"net/http"
)

func BenchmarkTest00202(w http.ResponseWriter, r *http.Request) {
	var req struct {
		TargetURL string `json:"target_url"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	tlsConfig := &tls.Config{
		MinVersion: tls.VersionTLS13,
		CipherSuites: []uint16{
			tls.TLS_AES_256_GCM_SHA384,
			tls.TLS_CHACHA20_POLY1305_SHA256,
		},
	}

	transport := &http.Transport{
		TLSClientConfig: tlsConfig,
	}

	client := &http.Client{
		Transport: transport,
	}

	resp, err := client.Get(req.TargetURL)
	if err != nil {
		http.Error(w, "request failed: "+err.Error(), http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"target_status": resp.StatusCode,
		"tls_version":   "1.3",
	})
}
