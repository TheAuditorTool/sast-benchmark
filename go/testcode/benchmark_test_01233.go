package testcode

import (
	"net/http"
)

var benchmarkTest01233Log []string

func BenchmarkTest01233(w http.ResponseWriter, r *http.Request) {
	entry := r.URL.Query().Get("entry")
	go func() {
		benchmarkTest01233Log = append(benchmarkTest01233Log, entry)
	}()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "logged"})
}
