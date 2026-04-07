package testcode

import (
	"net/http"
	"sync"
)

var benchmarkTest01260Mu sync.Mutex
var benchmarkTest01260Items []string

func BenchmarkTest01260(w http.ResponseWriter, r *http.Request) {
	item := r.URL.Query().Get("item")
	benchmarkTest01260Mu.Lock()
	benchmarkTest01260Items = append(benchmarkTest01260Items, item)
	benchmarkTest01260Mu.Unlock()
	RespondJSON(w, http.StatusOK, map[string]int{"total": len(benchmarkTest01260Items)})
}
