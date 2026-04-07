package testcode

import (
	"expvar"
	"net/http"
)

func BenchmarkTest01324(w http.ResponseWriter, r *http.Request) {
	expvar.Handler().ServeHTTP(w, r)
}
