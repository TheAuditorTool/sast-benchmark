package testcode

import (
	"io"
	"net/http"
)

func BenchmarkTest00137(w http.ResponseWriter, r *http.Request) {
	port := r.URL.Query().Get("port")
	targetURL := "http://internal:" + port + "/status"
	resp, err := http.Get(targetURL)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	RespondJSON(w, http.StatusOK, map[string]string{"status": string(body)})
}
