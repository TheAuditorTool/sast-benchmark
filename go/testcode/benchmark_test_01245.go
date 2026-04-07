package testcode

import (
	"net/http"
)

var benchmarkTest01245Resource int

func BenchmarkTest01245(w http.ResponseWriter, r *http.Request) {
	sem := make(chan struct{}, 2)
	sem <- struct{}{}
	defer func() { <-sem }()
	benchmarkTest01245Resource++
	RespondJSON(w, http.StatusOK, map[string]int{"val": benchmarkTest01245Resource})
}
