package testcode

import (
	"net/http"
	"sync/atomic"
)

type benchmarkTest01255Config struct{ debug bool }

var benchmarkTest01255AtomicCfg atomic.Value

func BenchmarkTest01255(w http.ResponseWriter, r *http.Request) {
	if r.Method == http.MethodPost {
		benchmarkTest01255AtomicCfg.Store(&benchmarkTest01255Config{debug: true})
		RespondJSON(w, http.StatusOK, map[string]string{"status": "updated"})
		return
	}
	cfg, _ := benchmarkTest01255AtomicCfg.Load().(*benchmarkTest01255Config)
	debug := cfg != nil && cfg.debug
	RespondJSON(w, http.StatusOK, map[string]bool{"debug": debug})
}
