package testcode

import "net/http"

var benchmarkTest01093Headers = map[string]string{
	"Authorization": "Bearer hardcoded-api-key-abc123",
	"Content-Type":  "application/json",
}

func BenchmarkTest01093(w http.ResponseWriter, r *http.Request) {
	RespondJSON(w, http.StatusOK, benchmarkTest01093Headers)
}
