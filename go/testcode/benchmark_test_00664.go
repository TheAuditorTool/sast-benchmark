package testcode

import (
	"net/http"
	"sync"
)

var (
	benchmarkTest00664Mu    sync.RWMutex
	benchmarkTest00664Cache = make(map[string]string)
)

func BenchmarkTest00664(w http.ResponseWriter, r *http.Request) {
	key := r.URL.Query().Get("key")
	value := r.URL.Query().Get("value")

	if key == "" {
		http.Error(w, "key required", http.StatusBadRequest)
		return
	}

	if value != "" {
		benchmarkTest00664Mu.Lock()
		benchmarkTest00664Cache[key] = value
		benchmarkTest00664Mu.Unlock()

		RespondJSON(w, http.StatusOK, map[string]string{"status": "cached"})
		return
	}

	benchmarkTest00664Mu.RLock()
	cached, ok := benchmarkTest00664Cache[key]
	benchmarkTest00664Mu.RUnlock()

	if !ok {
		RespondJSON(w, http.StatusNotFound, map[string]string{"error": "not found"})
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"key": key, "value": cached})
}
