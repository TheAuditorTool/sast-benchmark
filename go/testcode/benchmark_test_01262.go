package testcode

import (
	"net/http"
	"sync"
)

var benchmarkTest01262Pool = sync.Pool{
	New: func() interface{} { return make([]byte, 512) },
}

func BenchmarkTest01262(w http.ResponseWriter, r *http.Request) {
	buf := benchmarkTest01262Pool.Get().([]byte)
	defer benchmarkTest01262Pool.Put(buf)
	n, _ := r.Body.Read(buf)
	RespondJSON(w, http.StatusOK, map[string]int{"read": n})
}
