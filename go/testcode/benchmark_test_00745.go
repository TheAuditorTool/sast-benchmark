package testcode

import (
	"net/http"
	"net/url"
	"os"
)

func BenchmarkTest00745(w http.ResponseWriter, r *http.Request) {
	userPath := r.URL.Query().Get("path")
	parsed, err := url.Parse(userPath)
	if err != nil || parsed.Host != "" || parsed.Scheme != "" {
		http.Error(w, "only relative paths allowed", http.StatusBadRequest)
		return
	}
	baseURL := os.Getenv("API_BASE_URL")
	resp, err := http.Get(baseURL + parsed.Path)
	if err != nil {
		http.Error(w, "fetch error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
