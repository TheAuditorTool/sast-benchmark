package testcode

import (
	"io"
	"net/http"
	"os"
)

func BenchmarkTest00143(w http.ResponseWriter, r *http.Request) {
	apiURL := os.Getenv("EXTERNAL_API_URL")
	if apiURL == "" {
		http.Error(w, "api not configured", http.StatusInternalServerError)
		return
	}
	resp, err := http.Get(apiURL)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	RespondJSON(w, http.StatusOK, map[string]string{"data": string(body)})
}
