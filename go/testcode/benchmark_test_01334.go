package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest01334(w http.ResponseWriter, r *http.Request) {
	for name, values := range r.Header {
		for _, v := range values {
			fmt.Fprintf(w, "%s: %s\n", name, v)
		}
	}
}
