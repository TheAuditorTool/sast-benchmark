package testcode

import (
	"net/http"
	"net/http/pprof"
)

var benchmarkTest01340AdminToken = "secret-admin-token"

func BenchmarkTest01340(w http.ResponseWriter, r *http.Request) {
	if r.Header.Get("X-Admin-Token") != benchmarkTest01340AdminToken {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	pprof.Handler("heap").ServeHTTP(w, r)
}
