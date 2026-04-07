package testcode

import (
	"net/http"
	"sync/atomic"
)

var benchmarkTest01251Counter atomic.Int64

func BenchmarkTest01251(w http.ResponseWriter, r *http.Request) {
	n := benchmarkTest01251Counter.Add(1)
	RespondJSON(w, http.StatusOK, map[string]int64{"requests": n})
}
