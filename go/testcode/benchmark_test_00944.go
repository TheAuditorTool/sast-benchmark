package testcode

import (
	"encoding/json"
	"net/http"
)

func BenchmarkTest00944(w http.ResponseWriter, r *http.Request) {
	dec := json.NewDecoder(r.Body)
	dec.UseNumber()
	var data interface{}
	if err := dec.Decode(&data); err != nil {
		http.Error(w, "decode error", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]interface{}{"data": data})
}
