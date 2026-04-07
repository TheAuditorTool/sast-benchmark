package testcode

import (
	"net/http"
)

func BenchmarkTest00716(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("search")
	RespondJSON(w, http.StatusOK, map[string]string{"query": input})
}
