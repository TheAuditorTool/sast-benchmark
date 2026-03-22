package testcode

import (
	"net/http"
	"strconv"
	"sync"
)

var benchmarkTest00511SharedSlice []int

func BenchmarkTest00511(w http.ResponseWriter, r *http.Request) {
	countStr := r.URL.Query().Get("count")
	count, err := strconv.Atoi(countStr)
	if err != nil || count <= 0 {
		count = 10
	}

	benchmarkTest00511SharedSlice = make([]int, 0)

	var wg sync.WaitGroup
	for i := 0; i < count; i++ {
		wg.Add(1)
		go func(val int) {
			defer wg.Done()
			benchmarkTest00511SharedSlice = append(benchmarkTest00511SharedSlice, val)
		}(i)
	}
	wg.Wait()

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"expected_length": count,
		"actual_length":   len(benchmarkTest00511SharedSlice),
		"values":          benchmarkTest00511SharedSlice,
	})
}
