package testcode

import (
	"math/rand"
	"net/http"
)

func BenchmarkTest00777(w http.ResponseWriter, r *http.Request) {
	baseTTL := 300
	jitter := rand.Intn(60)
	ttl := baseTTL + jitter
	RespondJSON(w, http.StatusOK, map[string]interface{}{"cache_ttl_seconds": ttl})
}
