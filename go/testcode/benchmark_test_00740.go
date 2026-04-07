package testcode

import (
	"net/http"
	"net/url"
	"os"
)

func BenchmarkTest00740(w http.ResponseWriter, r *http.Request) {
	baseURL := os.Getenv("UPSTREAM_API_URL")
	searchTerm := r.URL.Query().Get("q")
	parsed, _ := url.Parse(baseURL)
	q := parsed.Query()
	q.Set("search", searchTerm)
	parsed.RawQuery = q.Encode()
	resp, err := http.Get(parsed.String())
	if err != nil {
		http.Error(w, "fetch error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
