package testcode

import (
	"net/http"
)

func BenchmarkTest00675(w http.ResponseWriter, r *http.Request) {
	requestID := r.Header.Get("X-Request-Id")
	if requestID == "" {
		http.Error(w, "X-Request-Id header required", http.StatusBadRequest)
		return
	}

	headers := make(map[string][]string)
	for k, v := range r.Header {
		headers[k] = v
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"request_id": requestID,
		"headers":    headers,
	})
}
