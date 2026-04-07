package testcode

import (
	"encoding/json"
	"net/http"
)

type benchmarkTest01308Input struct {
	Name  string `json:"name"`
	Email string `json:"email"`
}

func BenchmarkTest01308(w http.ResponseWriter, r *http.Request) {
	var input benchmarkTest01308Input
	dec := json.NewDecoder(r.Body)
	dec.DisallowUnknownFields()
	if err := dec.Decode(&input); err != nil {
		http.Error(w, "invalid input: "+err.Error(), http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"name": input.Name})
}
