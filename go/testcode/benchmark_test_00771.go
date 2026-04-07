package testcode

import (
	"math/rand"
	"net/http"
	"time"
)

func BenchmarkTest00771(w http.ResponseWriter, r *http.Request) {
	jitter := time.Duration(rand.Intn(100)) * time.Millisecond
	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"retry_after_ms": jitter.Milliseconds(),
	})
}
