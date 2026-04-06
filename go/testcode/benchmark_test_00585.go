package testcode

import (
	"encoding/json"
	"net/http"
)

type benchmarkTest00585Request struct {
	Name  string `json:"name"`
	Email string `json:"email"`
}

func BenchmarkTest00585(w http.ResponseWriter, r *http.Request) {
	r.Body = http.MaxBytesReader(w, r.Body, 1<<20)
	dec := json.NewDecoder(r.Body)
	dec.DisallowUnknownFields()

	var req benchmarkTest00585Request
	if err := dec.Decode(&req); err != nil {
		http.Error(w, "invalid request body", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"name": req.Name, "email": req.Email})
}
