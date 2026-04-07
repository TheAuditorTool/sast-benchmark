package testcode

import (
	"net/http"
)

var benchmarkTest00746ServiceMap = map[string]string{
	"weather":  "https://weather.internal.example.com/api",
	"currency": "https://currency.internal.example.com/api",
	"stocks":   "https://stocks.internal.example.com/api",
}

func BenchmarkTest00746(w http.ResponseWriter, r *http.Request) {
	serviceKey := r.URL.Query().Get("service")
	target, ok := benchmarkTest00746ServiceMap[serviceKey]
	if !ok {
		http.Error(w, "unknown service", http.StatusBadRequest)
		return
	}
	resp, err := http.Get(target)
	if err != nil {
		http.Error(w, "fetch error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
