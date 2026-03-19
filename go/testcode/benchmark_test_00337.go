package testcode

import (
	"encoding/json"
	"net/http"
)

func BenchmarkTest00337(w http.ResponseWriter, r *http.Request) {
	var input struct {
		Query string `json:"query"`
	}
	json.NewDecoder(r.Body).Decode(&input)
	req := &BenchSvcRequest{Query: input.Query}
	err := BenchSvcProcessStructSafe(req)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
