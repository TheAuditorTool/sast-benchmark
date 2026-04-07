package testcode

import (
	"net/http"
	"sync"
)

func BenchmarkTest01246(w http.ResponseWriter, r *http.Request) {
	items := []string{"a", "b", "c"}
	var wg sync.WaitGroup
	results := make([]string, len(items))
	for i, item := range items {
		wg.Add(1)
		go func() {
			defer wg.Done()
			results[i] = item + "_done"
		}()
	}
	wg.Wait()
	RespondJSON(w, http.StatusOK, results)
}
