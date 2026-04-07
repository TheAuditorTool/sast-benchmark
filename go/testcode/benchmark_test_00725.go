package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00725(w http.ResponseWriter, r *http.Request) {
	host := r.Header.Get("X-Forwarded-Host")
	path := r.URL.Query().Get("path")
	target := fmt.Sprintf("https://%s%s", host, path)
	req, err := http.NewRequest("GET", target, nil)
	if err != nil {
		http.Error(w, "request error", http.StatusBadRequest)
		return
	}
	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		http.Error(w, "fetch error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
