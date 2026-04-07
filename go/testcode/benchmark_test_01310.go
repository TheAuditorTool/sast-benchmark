package testcode

import (
	"net/http"
	"strconv"
)

func BenchmarkTest01310(w http.ResponseWriter, r *http.Request) {
	n, err := strconv.ParseInt(r.URL.Query().Get("count"), 10, 64)
	if err != nil || n < 1 || n > 10000 {
		http.Error(w, "invalid count", http.StatusBadRequest)
		return
	}
	buf := make([]byte, n)
	RespondJSON(w, http.StatusOK, map[string]int{"size": len(buf)})
}
