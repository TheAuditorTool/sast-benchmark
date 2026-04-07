package testcode

import (
	"net/http"
	"strconv"
)

func benchmarkTest01322Recurse(n int) int {
	if n <= 0 {
		return 0
	}
	return benchmarkTest01322Recurse(n-1) + 1
}

func BenchmarkTest01322(w http.ResponseWriter, r *http.Request) {
	depth, _ := strconv.Atoi(r.URL.Query().Get("depth"))
	if depth > 100 {
		depth = 100
	}
	if depth < 0 {
		depth = 0
	}
	result := benchmarkTest01322Recurse(depth)
	RespondJSON(w, http.StatusOK, map[string]int{"result": result})
}
