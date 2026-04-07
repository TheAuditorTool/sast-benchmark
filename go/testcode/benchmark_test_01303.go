package testcode

import (
	"encoding/json"
	"net/http"
)

func BenchmarkTest01303(w http.ResponseWriter, r *http.Request) {
	var body map[string]json.Number
	dec := json.NewDecoder(r.Body)
	dec.UseNumber()
	if err := dec.Decode(&body); err != nil {
		http.Error(w, "bad json", http.StatusBadRequest)
		return
	}
	size, _ := body["size"].Int64()
	buf := make([]byte, size)
	RespondJSON(w, http.StatusOK, map[string]int{"allocated": len(buf)})
}
