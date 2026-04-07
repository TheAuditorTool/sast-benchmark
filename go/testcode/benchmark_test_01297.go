package testcode

import (
	"net/http"
	"strconv"
)

func BenchmarkTest01297(w http.ResponseWriter, r *http.Request) {
	n, _ := strconv.Atoi(r.URL.Query().Get("count"))
	total := n * 1000
	buf := make([]byte, total)
	RespondJSON(w, http.StatusOK, map[string]int{"size": len(buf)})
}
