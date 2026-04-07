package testcode

import (
	"fmt"
	"net/http"
	"strconv"
)

func BenchmarkTest00720(w http.ResponseWriter, r *http.Request) {
	raw := r.URL.Query().Get("page")
	page, err := strconv.Atoi(raw)
	if err != nil || page < 1 {
		page = 1
	}
	w.Header().Set("Content-Type", "text/html")
	fmt.Fprintf(w, "<p>Page %d of results</p>", page)
}
