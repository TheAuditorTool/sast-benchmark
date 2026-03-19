package testcode

import (
	"net/http"
)

func BenchmarkTest00074(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("input")
	if input == "" {
		http.Error(w, "empty input", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"received": input})
}
