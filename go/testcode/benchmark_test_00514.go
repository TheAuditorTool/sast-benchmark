package testcode

import (
	"net/http"
	"strconv"
	"sync"
	"sync/atomic"
)

var benchmarkTest00514Counter int64

func BenchmarkTest00514(w http.ResponseWriter, r *http.Request) {
	iterations := r.URL.Query().Get("n")
	n, err := strconv.Atoi(iterations)
	if err != nil || n <= 0 {
		n = 10
	}

	atomic.StoreInt64(&benchmarkTest00514Counter, 0)

	var wg sync.WaitGroup
	for i := 0; i < n; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			atomic.AddInt64(&benchmarkTest00514Counter, 1)
		}()
	}
	wg.Wait()

	final := atomic.LoadInt64(&benchmarkTest00514Counter)
	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"expected": n,
		"actual":   final,
	})
}
