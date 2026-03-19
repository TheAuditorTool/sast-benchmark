package testcode

import (
	"io"
	"net/http"
	"net/url"
)

func BenchmarkTest00145(w http.ResponseWriter, r *http.Request) {
	searchTerm := r.URL.Query().Get("q")
	targetURL := "https://api.com/search?q=" + url.QueryEscape(searchTerm)
	resp, err := http.Get(targetURL)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	RespondJSON(w, http.StatusOK, map[string]string{"results": string(body)})
}
