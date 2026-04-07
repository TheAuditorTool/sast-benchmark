package testcode

import (
	"encoding/json"
	"io"
	"net/http"
)

type benchmarkTest00954Data struct {
	Key   string `json:"key"`
	Value string `json:"value"`
}

func BenchmarkTest00954(w http.ResponseWriter, r *http.Request) {
	limited := io.LimitReader(r.Body, 1<<20)
	var d benchmarkTest00954Data
	if err := json.NewDecoder(limited).Decode(&d); err != nil {
		http.Error(w, "json error", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"key": d.Key})
}
