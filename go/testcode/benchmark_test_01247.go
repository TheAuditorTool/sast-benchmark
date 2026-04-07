package testcode

import (
	"net/http"
)

var benchmarkTest01247Counter int64

func BenchmarkTest01247(w http.ResponseWriter, r *http.Request) {
	go func() {
		benchmarkTest01247Counter++
	}()
	RespondJSON(w, http.StatusOK, map[string]int64{"count": benchmarkTest01247Counter})
}
