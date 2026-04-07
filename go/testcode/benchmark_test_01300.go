package testcode

import (
	"net/http"
	"strconv"
)

func BenchmarkTest01300(w http.ResponseWriter, r *http.Request) {
	n, _ := strconv.Atoi(r.URL.Query().Get("workers"))
	done := make(chan struct{})
	for i := 0; i < n; i++ {
		go func() { done <- struct{}{} }()
	}
	for i := 0; i < n; i++ {
		<-done
	}
	RespondJSON(w, http.StatusOK, map[string]int{"workers": n})
}
