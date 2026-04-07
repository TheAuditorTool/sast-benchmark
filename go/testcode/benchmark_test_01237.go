package testcode

import (
	"net/http"
	"sync"
)

var benchmarkTest01237Mu sync.RWMutex
var benchmarkTest01237Data = map[string]string{}

func BenchmarkTest01237(w http.ResponseWriter, r *http.Request) {
	key := r.URL.Query().Get("key")
	val := r.URL.Query().Get("val")
	benchmarkTest01237Mu.RLock()
	defer benchmarkTest01237Mu.RUnlock()
	benchmarkTest01237Data[key] = val
	RespondJSON(w, http.StatusOK, map[string]string{"status": "stored"})
}
