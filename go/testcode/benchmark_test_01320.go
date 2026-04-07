package testcode

import (
	"net/http"
	"strconv"
)

func BenchmarkTest01320(w http.ResponseWriter, r *http.Request) {
	n, _ := strconv.Atoi(r.URL.Query().Get("workers"))
	if n > 10 {
		n = 10
	}
	if n < 1 {
		n = 1
	}
	done := make(chan struct{}, n)
	for i := 0; i < n; i++ {
		go func() { done <- struct{}{} }()
	}
	for i := 0; i < n; i++ {
		<-done
	}
	RespondJSON(w, http.StatusOK, map[string]int{"workers": n})
}
