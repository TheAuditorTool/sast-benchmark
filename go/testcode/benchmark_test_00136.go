package testcode

import (
	"io"
	"net/http"
)

func BenchmarkTest00136(w http.ResponseWriter, r *http.Request) {
	host := r.URL.Query().Get("host")
	targetURL := "http://" + host + "/api/data"
	resp, err := http.Get(targetURL)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	RespondJSON(w, http.StatusOK, map[string]string{"data": string(body)})
}
