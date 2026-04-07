package testcode

import (
	"net/http"
	"net/url"
)

func BenchmarkTest00748(w http.ResponseWriter, r *http.Request) {
	rawURL := r.URL.Query().Get("url")
	parsed, err := url.Parse(rawURL)
	if err != nil || parsed.Scheme != "https" {
		http.Error(w, "only https urls allowed", http.StatusBadRequest)
		return
	}
	resp, err := http.Get(parsed.String())
	if err != nil {
		http.Error(w, "fetch error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
