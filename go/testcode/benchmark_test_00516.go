package testcode

import (
	"net/http"
	"sync"
)

var (
	benchmarkTest00516Once     sync.Once
	benchmarkTest00516Resource map[string]string
)

func benchmarkTest00516Init() {
	benchmarkTest00516Resource = map[string]string{
		"version": "1.0.0",
		"status":  "initialized",
		"name":    "shared-config",
	}
}

func BenchmarkTest00516(w http.ResponseWriter, r *http.Request) {
	var wg sync.WaitGroup
	wg.Add(5)

	for i := 0; i < 5; i++ {
		go func() {
			defer wg.Done()
			benchmarkTest00516Once.Do(benchmarkTest00516Init)
		}()
	}
	wg.Wait()

	key := r.URL.Query().Get("key")
	if val, ok := benchmarkTest00516Resource[key]; ok {
		RespondJSON(w, http.StatusOK, map[string]string{"key": key, "value": val})
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{"config": benchmarkTest00516Resource})
}
