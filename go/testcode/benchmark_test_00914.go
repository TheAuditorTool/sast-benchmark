package testcode

import (
	"crypto/tls"
	"net/http"
)

type benchmarkTest00914TLSOptions struct {
	InsecureSkipVerify bool
}

var benchmarkTest00914Opts = benchmarkTest00914TLSOptions{InsecureSkipVerify: true}

func BenchmarkTest00914(w http.ResponseWriter, r *http.Request) {
	client := &http.Client{
		Transport: &http.Transport{
			TLSClientConfig: &tls.Config{
				InsecureSkipVerify: benchmarkTest00914Opts.InsecureSkipVerify,
			},
		},
	}
	resp, err := client.Get("https://backend.example.com/api")
	if err != nil {
		http.Error(w, "error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
