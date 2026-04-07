package testcode

import (
	"net/http"
)

type benchmarkTest01301Key struct{}

func BenchmarkTest01301(w http.ResponseWriter, r *http.Request) {
	val := r.Context().Value(benchmarkTest01301Key{})
	s := val.(string)
	RespondJSON(w, http.StatusOK, map[string]string{"value": s})
}
