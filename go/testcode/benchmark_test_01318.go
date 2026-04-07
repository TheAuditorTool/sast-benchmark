package testcode

import (
	"encoding/json"
	"net/http"
)

func BenchmarkTest01318(w http.ResponseWriter, r *http.Request) {
	r.Body = http.MaxBytesReader(w, r.Body, 1<<20)
	var body map[string]string
	if err := json.NewDecoder(r.Body).Decode(&body); err != nil {
		http.Error(w, "request too large or invalid", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"fields": len(body)})
}
