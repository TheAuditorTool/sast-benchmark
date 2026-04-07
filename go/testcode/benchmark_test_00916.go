package testcode

import (
	"crypto/tls"
	"net/http"
	"sync"
)

func BenchmarkTest00916(w http.ResponseWriter, r *http.Request) {
	var statusStr string
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		client := &http.Client{
			Transport: &http.Transport{
				TLSClientConfig: &tls.Config{InsecureSkipVerify: true},
			},
		}
		resp, err := client.Get("https://api.example.com/status")
		if err == nil {
			statusStr = resp.Status
			resp.Body.Close()
		}
	}()
	wg.Wait()
	RespondJSON(w, http.StatusOK, map[string]string{"status": statusStr})
}
