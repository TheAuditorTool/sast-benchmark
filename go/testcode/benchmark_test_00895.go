package testcode

import (
	"net/http"
	"strconv"
)

var benchmarkTest00895Pages = []string{"/", "/about", "/products", "/contact"}

func BenchmarkTest00895(w http.ResponseWriter, r *http.Request) {
	idxStr := r.URL.Query().Get("page")
	idx, err := strconv.Atoi(idxStr)
	if err != nil || idx < 0 || idx >= len(benchmarkTest00895Pages) {
		idx = 0
	}
	http.Redirect(w, r, benchmarkTest00895Pages[idx], http.StatusFound)
}
