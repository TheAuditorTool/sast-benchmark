package testcode

import (
	"encoding/json"
	"net/http"
	"sync"
)

func BenchmarkTest00938(w http.ResponseWriter, r *http.Request) {
	var result interface{}
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		json.NewDecoder(r.Body).Decode(&result)
	}()
	wg.Wait()
	RespondJSON(w, http.StatusOK, map[string]interface{}{"data": result})
}
