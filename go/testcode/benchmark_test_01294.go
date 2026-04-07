package testcode

import (
	"net/http"
	"strconv"
)

func BenchmarkTest01294(w http.ResponseWriter, r *http.Request) {
	n, _ := strconv.Atoi(r.URL.Query().Get("size"))
	buf := make([]byte, n)
	RespondJSON(w, http.StatusOK, map[string]int{"allocated": len(buf)})
}
