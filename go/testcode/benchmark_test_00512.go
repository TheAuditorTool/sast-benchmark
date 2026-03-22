package testcode

import (
	"net/http"
	"sync"
)

var (
	benchmarkTest00512Map = make(map[string]int)
	benchmarkTest00512Mu  sync.Mutex
)

func BenchmarkTest00512(w http.ResponseWriter, r *http.Request) {
	key := r.URL.Query().Get("key")
	value := 1

	var wg sync.WaitGroup
	wg.Add(2)

	go func() {
		defer wg.Done()
		benchmarkTest00512Mu.Lock()
		benchmarkTest00512Map[key] = value
		benchmarkTest00512Mu.Unlock()
	}()

	go func() {
		defer wg.Done()
		benchmarkTest00512Mu.Lock()
		benchmarkTest00512Map[key] = value + 1
		benchmarkTest00512Mu.Unlock()
	}()

	wg.Wait()

	benchmarkTest00512Mu.Lock()
	result := benchmarkTest00512Map[key]
	benchmarkTest00512Mu.Unlock()

	RespondJSON(w, http.StatusOK, map[string]interface{}{"key": key, "value": result})
}
