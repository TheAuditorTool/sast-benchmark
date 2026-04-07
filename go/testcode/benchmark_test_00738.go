package testcode

import (
	"net/http"
	"strings"
)

const benchmarkTest00738BaseURL = "https://api.internal.example.com"

func BenchmarkTest00738(w http.ResponseWriter, r *http.Request) {
	path := r.URL.Query().Get("path")
	if !strings.HasPrefix(path, "/") {
		path = "/" + path
	}
	resp, err := http.Get(benchmarkTest00738BaseURL + path)
	if err != nil {
		http.Error(w, "fetch error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
