package testcode

import (
	"crypto/tls"
	"net/http"
	"os"
)

func BenchmarkTest00912(w http.ResponseWriter, r *http.Request) {
	skipVerify := os.Getenv("SKIP_TLS_VERIFY") == "true"
	client := &http.Client{
		Transport: &http.Transport{
			TLSClientConfig: &tls.Config{InsecureSkipVerify: skipVerify},
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
