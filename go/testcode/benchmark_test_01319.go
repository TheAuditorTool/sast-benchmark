package testcode

import (
	"net/http"
	"strconv"
)

func BenchmarkTest01319(w http.ResponseWriter, r *http.Request) {
	n, err := strconv.ParseInt(r.URL.Query().Get("page"), 10, 32)
	if err != nil || n < 1 || n > 100 {
		http.Error(w, "page must be between 1 and 100", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int64{"page": n})
}
