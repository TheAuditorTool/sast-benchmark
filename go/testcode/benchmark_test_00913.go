package testcode

import (
	"crypto/tls"
	"net/http"
)

var benchmarkTest00913Client = &http.Client{Transport: &http.Transport{}}

func init() {
	if t, ok := benchmarkTest00913Client.Transport.(*http.Transport); ok {
		t.TLSClientConfig = &tls.Config{InsecureSkipVerify: true}
	}
}

func BenchmarkTest00913(w http.ResponseWriter, r *http.Request) {
	resp, err := benchmarkTest00913Client.Get("https://service.example.com/status")
	if err != nil {
		http.Error(w, "error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
