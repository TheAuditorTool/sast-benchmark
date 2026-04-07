package testcode

import (
	"net/http"
	"sync"
)

var benchmarkTest01249RWMu sync.RWMutex
var benchmarkTest01249Cache = map[string]string{}

func BenchmarkTest01249(w http.ResponseWriter, r *http.Request) {
	key := r.URL.Query().Get("key")
	val := r.URL.Query().Get("val")
	if val == "" {
		benchmarkTest01249RWMu.RLock()
		v := benchmarkTest01249Cache[key]
		benchmarkTest01249RWMu.RUnlock()
		RespondJSON(w, http.StatusOK, map[string]string{"val": v})
		return
	}
	benchmarkTest01249RWMu.Lock()
	benchmarkTest01249Cache[key] = val
	benchmarkTest01249RWMu.Unlock()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "stored"})
}
