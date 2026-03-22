package testcode

import (
	"net/http"
	"strconv"
	"sync"
)

func benchmarkTest00515Worker(id int, results chan<- map[string]interface{}) {
	results <- map[string]interface{}{
		"worker_id": id,
		"result":    id * id,
	}
}

func BenchmarkTest00515(w http.ResponseWriter, r *http.Request) {
	countStr := r.URL.Query().Get("workers")
	count, err := strconv.Atoi(countStr)
	if err != nil || count <= 0 {
		count = 5
	}

	results := make(chan map[string]interface{}, count)

	var wg sync.WaitGroup
	for i := 0; i < count; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()
			benchmarkTest00515Worker(id, results)
		}(i)
	}

	go func() {
		wg.Wait()
		close(results)
	}()

	var collected []map[string]interface{}
	for res := range results {
		collected = append(collected, res)
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{"results": collected})
}
