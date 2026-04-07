package testcode

import (
	"encoding/gob"
	"net/http"
)

type benchmarkTest00943TypeA struct{ Name string }
type benchmarkTest00943TypeB struct{ Cmd string }

func init() {
	gob.Register(&benchmarkTest00943TypeA{})
	gob.Register(&benchmarkTest00943TypeB{})
}

func BenchmarkTest00943(w http.ResponseWriter, r *http.Request) {
	var obj interface{}
	if err := gob.NewDecoder(r.Body).Decode(&obj); err != nil {
		http.Error(w, "decode error", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]interface{}{"data": obj})
}
