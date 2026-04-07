package testcode

import (
	"net/http"
	"sync"
)

type benchmarkTest01252Singleton struct{ name string }

var benchmarkTest01252Once sync.Once
var benchmarkTest01252Inst *benchmarkTest01252Singleton

func BenchmarkTest01252(w http.ResponseWriter, r *http.Request) {
	benchmarkTest01252Once.Do(func() {
		benchmarkTest01252Inst = &benchmarkTest01252Singleton{name: "app"}
	})
	RespondJSON(w, http.StatusOK, map[string]string{"name": benchmarkTest01252Inst.name})
}
