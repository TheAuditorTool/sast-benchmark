package testcode

import (
	"net/http"
	"strconv"
	"sync"
)

var (
	benchmarkTest00513State   = make(map[string]int)
	benchmarkTest00513RWMutex sync.RWMutex
)

func BenchmarkTest00513(w http.ResponseWriter, r *http.Request) {
	action := r.URL.Query().Get("action")
	key := r.URL.Query().Get("key")

	if action == "write" {
		valStr := r.URL.Query().Get("value")
		val, err := strconv.Atoi(valStr)
		if err != nil {
			val = 0
		}

		benchmarkTest00513RWMutex.Lock()
		benchmarkTest00513State[key] = val
		benchmarkTest00513RWMutex.Unlock()

		RespondJSON(w, http.StatusOK, map[string]string{"status": "written"})
		return
	}

	benchmarkTest00513RWMutex.RLock()
	val, exists := benchmarkTest00513State[key]
	benchmarkTest00513RWMutex.RUnlock()

	if !exists {
		RespondJSON(w, http.StatusNotFound, map[string]string{"error": "key not found"})
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{"key": key, "value": val})
}
