package testcode

import (
	"net/http"
)

type benchmarkTest01315Key struct{}

func BenchmarkTest01315(w http.ResponseWriter, r *http.Request) {
	val, ok := r.Context().Value(benchmarkTest01315Key{}).(string)
	if !ok {
		http.Error(w, "missing or invalid context value", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"value": val})
}
