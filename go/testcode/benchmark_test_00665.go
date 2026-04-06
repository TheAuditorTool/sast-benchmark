package testcode

import (
	"net/http"
	"sync/atomic"
)

type benchmarkTest00665Settings struct {
	MaxConnections int
	RateLimit      int
	FeatureFlags   map[string]bool
}

var benchmarkTest00665Config atomic.Value

func init() {
	benchmarkTest00665Config.Store(&benchmarkTest00665Settings{
		MaxConnections: 100,
		RateLimit:      1000,
		FeatureFlags:   map[string]bool{"beta": false, "metrics": true},
	})
}

func BenchmarkTest00665(w http.ResponseWriter, r *http.Request) {
	action := r.URL.Query().Get("action")

	if action == "reload" {
		benchmarkTest00665Config.Store(&benchmarkTest00665Settings{
			MaxConnections: 200,
			RateLimit:      2000,
			FeatureFlags:   map[string]bool{"beta": true, "metrics": true},
		})
		RespondJSON(w, http.StatusOK, map[string]string{"status": "config reloaded"})
		return
	}

	config := benchmarkTest00665Config.Load().(*benchmarkTest00665Settings)

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"max_connections": config.MaxConnections,
		"rate_limit":      config.RateLimit,
		"feature_flags":   config.FeatureFlags,
	})
}
