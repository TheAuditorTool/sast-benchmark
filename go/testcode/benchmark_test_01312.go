package testcode

import (
	"net/http"
	"strconv"
)

func BenchmarkTest01312(w http.ResponseWriter, r *http.Request) {
	n, err := strconv.Atoi(r.URL.Query().Get("size"))
	if err != nil || n < 1 || n > 1000 {
		http.Error(w, "size must be between 1 and 1000", http.StatusBadRequest)
		return
	}
	buf := make([]byte, n)
	RespondJSON(w, http.StatusOK, map[string]int{"allocated": len(buf)})
}
