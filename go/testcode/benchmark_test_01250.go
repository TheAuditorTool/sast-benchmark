package testcode

import (
	"net/http"
	"sync"
)

var benchmarkTest01250Map sync.Map

func BenchmarkTest01250(w http.ResponseWriter, r *http.Request) {
	key := r.URL.Query().Get("key")
	val := r.URL.Query().Get("val")
	if val == "" {
		v, _ := benchmarkTest01250Map.Load(key)
		RespondJSON(w, http.StatusOK, map[string]interface{}{"val": v})
		return
	}
	benchmarkTest01250Map.Store(key, val)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "stored"})
}
