package testcode

import (
	"net/http"
	"os"
)

var benchmarkTest01110APIKey string

func init() {
	benchmarkTest01110APIKey = os.Getenv("SERVICE_API_KEY")
}

func BenchmarkTest01110(w http.ResponseWriter, r *http.Request) {
	if benchmarkTest01110APIKey == "" {
		http.Error(w, "service not configured", http.StatusServiceUnavailable)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ready"})
}
