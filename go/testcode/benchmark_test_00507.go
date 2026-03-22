package testcode

import (
	"net/http"
	"sync"
)

var benchmarkTest00507SharedMap = make(map[string]int)

func BenchmarkTest00507(w http.ResponseWriter, r *http.Request) {
	key := r.URL.Query().Get("key")
	value := 1

	var wg sync.WaitGroup
	wg.Add(2)

	go func() {
		defer wg.Done()
		benchmarkTest00507SharedMap[key] = value
	}()

	go func() {
		defer wg.Done()
		benchmarkTest00507SharedMap[key] = value + 1
	}()

	wg.Wait()

	result := benchmarkTest00507SharedMap[key]
	RespondJSON(w, http.StatusOK, map[string]interface{}{"key": key, "value": result})
}
