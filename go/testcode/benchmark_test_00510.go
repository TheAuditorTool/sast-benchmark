package testcode

import (
	"fmt"
	"net/http"
	"strings"
	"sync"
)

func benchmarkTest00510Process(item string, index int) string {
	return fmt.Sprintf("%d:%s", index, item)
}

func BenchmarkTest00510(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("items")
	items := strings.Split(input, ",")

	var mu sync.Mutex
	results := make([]string, 0, len(items))

	var wg sync.WaitGroup
	for i, v := range items {
		wg.Add(1)
		go func() {
			defer wg.Done()
			result := benchmarkTest00510Process(v, i)
			mu.Lock()
			results = append(results, result)
			mu.Unlock()
		}()
	}
	wg.Wait()

	RespondJSON(w, http.StatusOK, map[string]interface{}{"results": results})
}
