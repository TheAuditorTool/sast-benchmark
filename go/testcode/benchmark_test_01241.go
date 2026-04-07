package testcode

import (
	"net/http"
)

type benchmarkTest01241Obj struct{ ready bool }

var benchmarkTest01241Instance *benchmarkTest01241Obj

func BenchmarkTest01241(w http.ResponseWriter, r *http.Request) {
	if benchmarkTest01241Instance == nil {
		benchmarkTest01241Instance = &benchmarkTest01241Obj{ready: true}
	}
	RespondJSON(w, http.StatusOK, map[string]bool{"ready": benchmarkTest01241Instance.ready})
}
