package testcode

import (
	"net/http"
	"net/url"
)

func BenchmarkTest00734(w http.ResponseWriter, r *http.Request) {
	userHost := r.URL.Query().Get("host")
	userPath := r.URL.Query().Get("path")
	target := &url.URL{
		Scheme: "http",
		Host:   userHost,
		Path:   userPath,
	}
	resp, err := http.Get(target.String())
	if err != nil {
		http.Error(w, "fetch error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
