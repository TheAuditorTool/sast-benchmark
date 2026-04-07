package testcode

import (
	"fmt"
	"net/http"
	"os"
)

func BenchmarkTest01337(w http.ResponseWriter, r *http.Request) {
	for _, env := range os.Environ() {
		fmt.Fprintln(w, env)
	}
}
