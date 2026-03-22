package testcode

import (
	"io"
	"net/http"
	"net/url"
)

var benchmarkTest00531AllowedSchemes = map[string]bool{
	"http":  true,
	"https": true,
}

func BenchmarkTest00531(w http.ResponseWriter, r *http.Request) {
	target := r.URL.Query().Get("target")
	if target == "" {
		http.Error(w, "target URL required", http.StatusBadRequest)
		return
	}

	parsedURL, err := url.Parse(target)
	if err != nil {
		http.Error(w, "invalid URL", http.StatusBadRequest)
		return
	}

	if !benchmarkTest00531AllowedSchemes[parsedURL.Scheme] {
		http.Error(w, "only http and https schemes are allowed", http.StatusBadRequest)
		return
	}

	resp, err := http.Get(parsedURL.String())
	if err != nil {
		http.Error(w, "request failed", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		http.Error(w, "read error", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", resp.Header.Get("Content-Type"))
	w.WriteHeader(resp.StatusCode)
	w.Write(body)
}
