package testcode

import (
	"net/http"
	"net/url"
	"strings"
)

func BenchmarkTest00750(w http.ResponseWriter, r *http.Request) {
	rawURL := r.URL.Query().Get("endpoint")
	parsed, err := url.Parse(rawURL)
	if err != nil || (parsed.Scheme != "" && parsed.Scheme != "https") {
		http.Error(w, "invalid endpoint", http.StatusBadRequest)
		return
	}
	if parsed.Host != "" && !strings.HasSuffix(parsed.Host, ".internal.example.com") {
		http.Error(w, "external hosts not allowed", http.StatusForbidden)
		return
	}
	resp, err := http.Get("https://gateway.internal.example.com" + parsed.Path)
	if err != nil {
		http.Error(w, "fetch error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
