package testcode

import (
	"net/http"
	"sync"
)

type benchmarkTest01243Cache struct{ data string }

var benchmarkTest01243Mu sync.Mutex
var benchmarkTest01243CachePtr *benchmarkTest01243Cache

func BenchmarkTest01243(w http.ResponseWriter, r *http.Request) {
	if benchmarkTest01243CachePtr == nil {
		benchmarkTest01243Mu.Lock()
		if benchmarkTest01243CachePtr == nil {
			benchmarkTest01243CachePtr = &benchmarkTest01243Cache{data: "loaded"}
		}
		benchmarkTest01243Mu.Unlock()
	}
	RespondJSON(w, http.StatusOK, map[string]string{"data": benchmarkTest01243CachePtr.data})
}
