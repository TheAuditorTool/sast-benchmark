package testcode

import (
	"net/http"
	"sync"
)

var (
	benchmarkTest00662Mu    sync.RWMutex
	benchmarkTest00662Cache = make(map[string]string)
)

func BenchmarkTest00662(w http.ResponseWriter, r *http.Request) {
	key := r.URL.Query().Get("key")
	value := r.URL.Query().Get("value")

	if key == "" {
		http.Error(w, "key required", http.StatusBadRequest)
		return
	}

	if value != "" {
		benchmarkTest00662Mu.RLock()
		benchmarkTest00662Cache[key] = value
		benchmarkTest00662Mu.RUnlock()

		RespondJSON(w, http.StatusOK, map[string]string{"status": "cached"})
		return
	}

	benchmarkTest00662Mu.RLock()
	cached, ok := benchmarkTest00662Cache[key]
	benchmarkTest00662Mu.RUnlock()

	if !ok {
		RespondJSON(w, http.StatusNotFound, map[string]string{"error": "not found"})
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"key": key, "value": cached})
}
