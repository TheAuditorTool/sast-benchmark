package testcode

import (
	"net/http"
	"strconv"
	"sync"
)

var benchmarkTest00509Counter int

func BenchmarkTest00509(w http.ResponseWriter, r *http.Request) {
	iterations := r.URL.Query().Get("n")
	n, err := strconv.Atoi(iterations)
	if err != nil || n <= 0 {
		n = 10
	}

	benchmarkTest00509Counter = 0

	var wg sync.WaitGroup
	for i := 0; i < n; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			benchmarkTest00509Counter++
		}()
	}
	wg.Wait()

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"expected": n,
		"actual":   benchmarkTest00509Counter,
	})
}
