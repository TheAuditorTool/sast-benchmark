package testcode

import (
	"encoding/gob"
	"net/http"
)

func BenchmarkTest00487(w http.ResponseWriter, r *http.Request) {
	var result interface{}
	decoder := gob.NewDecoder(r.Body)
	err := decoder.Decode(&result)
	if err != nil {
		http.Error(w, "decode error", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]interface{}{"decoded": result})
}
