package testcode

import (
	"encoding/gob"
	"net/http"
)

func BenchmarkTest00933(w http.ResponseWriter, r *http.Request) {
	var data interface{}
	dec := gob.NewDecoder(r.Body)
	if err := dec.Decode(&data); err != nil {
		http.Error(w, "decode error", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]interface{}{"data": data})
}
