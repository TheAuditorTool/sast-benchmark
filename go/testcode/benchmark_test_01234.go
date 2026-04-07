package testcode

import (
	"net/http"
)

var benchmarkTest01234Config = map[string]string{"theme": "default"}

func BenchmarkTest01234(w http.ResponseWriter, r *http.Request) {
	key := r.URL.Query().Get("key")
	val := r.URL.Query().Get("val")
	current := benchmarkTest01234Config[key]
	benchmarkTest01234Config[key] = val
	RespondJSON(w, http.StatusOK, map[string]string{"old": current})
}
