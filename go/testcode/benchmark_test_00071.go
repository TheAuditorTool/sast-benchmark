package testcode

import (
	"net/http"
)

func BenchmarkTest00071(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("data")
	RespondJSON(w, http.StatusOK, map[string]string{"echo": input})
}
