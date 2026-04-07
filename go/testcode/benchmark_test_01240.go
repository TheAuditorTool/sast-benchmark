package testcode

import (
	"net/http"
)

var benchmarkTest01240Visits = map[string]int{}

func BenchmarkTest01240(w http.ResponseWriter, r *http.Request) {
	page := r.URL.Query().Get("page")
	benchmarkTest01240Visits[page]++
	RespondJSON(w, http.StatusOK, map[string]int{"visits": benchmarkTest01240Visits[page]})
}
