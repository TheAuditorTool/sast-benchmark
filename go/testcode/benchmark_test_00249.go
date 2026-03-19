package testcode

import (
	"encoding/gob"
	"fmt"
	"net/http"
)

func BenchmarkTest00249(w http.ResponseWriter, r *http.Request) {
	var payload interface{}

	decoder := gob.NewDecoder(r.Body)
	err := decoder.Decode(&payload)
	if err != nil {
		http.Error(w, "decode error", http.StatusBadRequest)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"received": fmt.Sprintf("%v", payload),
	})
}
