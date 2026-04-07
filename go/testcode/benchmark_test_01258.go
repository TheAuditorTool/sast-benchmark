package testcode

import (
	"net/http"
	"sync"
)

func BenchmarkTest01258(w http.ResponseWriter, r *http.Request) {
	var wg sync.WaitGroup
	results := make([]int, 4)
	for i := 0; i < 4; i++ {
		wg.Add(1)
		idx := i
		go func() {
			defer wg.Done()
			results[idx] = idx * idx
		}()
	}
	wg.Wait()
	RespondJSON(w, http.StatusOK, results)
}
