package testcode

import (
	"io"
	"net/http"
)

func BenchmarkTest00141(w http.ResponseWriter, r *http.Request) {
	resp, err := http.Get("https://api.trusted.com/data")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	RespondJSON(w, http.StatusOK, map[string]string{"data": string(body)})
}
