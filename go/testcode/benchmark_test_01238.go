package testcode

import (
	"net/http"
	"sync"
)

func BenchmarkTest01238(w http.ResponseWriter, r *http.Request) {
	counter := 0
	var wg sync.WaitGroup
	for i := 0; i < 5; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			counter++
		}()
	}
	wg.Wait()
	RespondJSON(w, http.StatusOK, map[string]int{"count": counter})
}
