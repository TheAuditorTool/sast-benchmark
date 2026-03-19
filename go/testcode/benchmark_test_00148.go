package testcode

import (
	"io"
	"net/http"
)

var knownEndpoints = map[string]string{
	"users":    "https://api.internal.com/v1/users",
	"products": "https://api.internal.com/v1/products",
	"orders":   "https://api.internal.com/v1/orders",
}

func BenchmarkTest00148(w http.ResponseWriter, r *http.Request) {
	key := r.URL.Query().Get("resource")
	targetURL, ok := knownEndpoints[key]
	if !ok {
		http.Error(w, "unknown resource", http.StatusBadRequest)
		return
	}
	resp, err := http.Get(targetURL)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	RespondJSON(w, http.StatusOK, map[string]string{"data": string(body)})
}
