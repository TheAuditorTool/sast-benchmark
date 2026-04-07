package testcode

import (
	"net/http"
)

type benchmarkTest01239State struct {
	count int
}

var benchmarkTest01239State01 = &benchmarkTest01239State{}

func BenchmarkTest01239(w http.ResponseWriter, r *http.Request) {
	go func() {
		benchmarkTest01239State01.count++
	}()
	RespondJSON(w, http.StatusOK, map[string]int{"count": benchmarkTest01239State01.count})
}
