package testcode

import (
	"encoding/json"
	"net/http"
)

type benchmarkTest00951Payload struct {
	ID    int    `json:"id"`
	Title string `json:"title"`
}

func BenchmarkTest00951(w http.ResponseWriter, r *http.Request) {
	var p benchmarkTest00951Payload
	dec := json.NewDecoder(r.Body)
	dec.DisallowUnknownFields()
	if err := dec.Decode(&p); err != nil {
		http.Error(w, "json error", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]interface{}{"id": p.ID})
}
