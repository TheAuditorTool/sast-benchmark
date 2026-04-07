package testcode

import (
	"crypto/tls"
	"net/http"
	"os"
)

func BenchmarkTest00923(w http.ResponseWriter, r *http.Request) {
	cert, err := tls.LoadX509KeyPair(os.Getenv("CLIENT_CERT"), os.Getenv("CLIENT_KEY"))
	if err != nil {
		http.Error(w, "cert load error", http.StatusInternalServerError)
		return
	}
	client := &http.Client{
		Transport: &http.Transport{
			TLSClientConfig: &tls.Config{
				Certificates: []tls.Certificate{cert},
				MinVersion:   tls.VersionTLS12,
			},
		},
	}
	resp, err := client.Get("https://mtls.example.com/api")
	if err != nil {
		http.Error(w, "error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
