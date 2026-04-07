package testcode

import (
	"net/http"
	"sync"
)

var benchmarkTest01248Mu sync.Mutex
var benchmarkTest01248Store = map[string]string{}

func BenchmarkTest01248(w http.ResponseWriter, r *http.Request) {
	key := r.URL.Query().Get("key")
	val := r.URL.Query().Get("val")
	benchmarkTest01248Mu.Lock()
	benchmarkTest01248Store[key] = val
	benchmarkTest01248Mu.Unlock()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "stored"})
}
