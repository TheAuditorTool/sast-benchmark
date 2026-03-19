package testcode

import (
	"io"
	"net/http"
)

func BenchmarkTest00131(w http.ResponseWriter, r *http.Request) {
	targetURL := r.URL.Query().Get("url")
	resp, err := http.Get(targetURL)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	RespondJSON(w, http.StatusOK, map[string]string{"result": string(body)})
}
