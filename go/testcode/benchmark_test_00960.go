package testcode

import (
	"encoding/gob"
	"net/http"
	"sync"
)

type benchmarkTest00960Metric struct {
	Name  string
	Value float64
}

func BenchmarkTest00960(w http.ResponseWriter, r *http.Request) {
	var m benchmarkTest00960Metric
	var wg sync.WaitGroup
	var decErr error
	wg.Add(1)
	go func() {
		defer wg.Done()
		decErr = gob.NewDecoder(r.Body).Decode(&m)
	}()
	wg.Wait()
	if decErr != nil {
		http.Error(w, "decode error", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]interface{}{"name": m.Name, "value": m.Value})
}
