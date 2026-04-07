package testcode

import (
	"net/http"
	"runtime"
)

func BenchmarkTest01325(w http.ResponseWriter, r *http.Request) {
	RespondJSON(w, http.StatusOK, map[string]int{"cpus": runtime.GOMAXPROCS(-1)})
}
