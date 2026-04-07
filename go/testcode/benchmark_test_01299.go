package testcode

import (
	"net/http"
	"strconv"
)

func benchmarkTest01299Recurse(n int) int {
	if n <= 0 {
		return 0
	}
	return benchmarkTest01299Recurse(n-1) + 1
}

func BenchmarkTest01299(w http.ResponseWriter, r *http.Request) {
	depth, _ := strconv.Atoi(r.URL.Query().Get("depth"))
	result := benchmarkTest01299Recurse(depth)
	RespondJSON(w, http.StatusOK, map[string]int{"result": result})
}
