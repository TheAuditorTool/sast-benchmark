package testcode

import (
	"net/http"
	"sync/atomic"
)

var benchmarkTest01259Hits int64

func BenchmarkTest01259(w http.ResponseWriter, r *http.Request) {
	n := atomic.AddInt64(&benchmarkTest01259Hits, 1)
	RespondJSON(w, http.StatusOK, map[string]int64{"hits": n})
}
