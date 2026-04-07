package testcode

import (
	"crypto/tls"
	"net/http"
)

type benchmarkTest00909Config struct {
	SkipVerify bool
	BaseURL    string
}

func BenchmarkTest00909(w http.ResponseWriter, r *http.Request) {
	cfg := benchmarkTest00909Config{SkipVerify: true, BaseURL: "https://backend.example.com"}
	client := &http.Client{
		Transport: &http.Transport{
			TLSClientConfig: &tls.Config{InsecureSkipVerify: cfg.SkipVerify},
		},
	}
	resp, err := client.Get(cfg.BaseURL + "/health")
	if err != nil {
		http.Error(w, "error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
