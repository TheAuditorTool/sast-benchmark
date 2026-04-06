package testcode

import (
	"net/http"
)

func BenchmarkTest00684(w http.ResponseWriter, r *http.Request) {
	if err := DB.PingContext(r.Context()); err != nil {
		RespondJSON(w, http.StatusServiceUnavailable, map[string]string{
			"status": "unavailable",
		})
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"status": "ok",
	})
}
