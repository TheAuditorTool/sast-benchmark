package testcode

import (
	"net/http"
	"strconv"
)

func BenchmarkTest01305(w http.ResponseWriter, r *http.Request) {
	n, _ := strconv.Atoi(r.Header.Get("X-Buffer-Size"))
	buf := make([]byte, n)
	RespondJSON(w, http.StatusOK, map[string]int{"size": len(buf)})
}
