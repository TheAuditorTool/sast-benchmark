package testcode

import (
	"encoding/json"
	"net/http"
)

type benchmarkTest00959Tag struct {
	Name  string `json:"name"`
	Color string `json:"color"`
}

func BenchmarkTest00959(w http.ResponseWriter, r *http.Request) {
	var tags []benchmarkTest00959Tag
	if err := json.NewDecoder(r.Body).Decode(&tags); err != nil {
		http.Error(w, "json error", http.StatusBadRequest)
		return
	}
	names := make([]string, 0, len(tags))
	for _, t := range tags {
		names = append(names, t.Name)
	}
	RespondJSON(w, http.StatusOK, map[string]interface{}{"count": len(tags), "names": names})
}
