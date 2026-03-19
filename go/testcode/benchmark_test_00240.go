package testcode

import (
	"crypto/tls"
	"io"
	"net/http"
	"time"
)

func BenchmarkTest00240(w http.ResponseWriter, r *http.Request) {
	apiEndpoint := r.URL.Query().Get("endpoint")
	if apiEndpoint == "" {
		http.Error(w, "missing endpoint", http.StatusBadRequest)
		return
	}

	client := &http.Client{
		Timeout: 10 * time.Second,
		Transport: &http.Transport{
			TLSClientConfig: &tls.Config{InsecureSkipVerify: true},
		},
	}

	resp, err := client.Get(apiEndpoint)
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
