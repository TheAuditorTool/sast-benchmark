package testcode

import (
	"encoding/gob"
	"net/http"
)

type benchmarkTest00937Runner interface {
	Run() string
}

func BenchmarkTest00937(w http.ResponseWriter, r *http.Request) {
	var obj interface{}
	if err := gob.NewDecoder(r.Body).Decode(&obj); err != nil {
		http.Error(w, "decode error", http.StatusBadRequest)
		return
	}
	if runner, ok := obj.(benchmarkTest00937Runner); ok {
		result := runner.Run()
		RespondJSON(w, http.StatusOK, map[string]string{"result": result})
		return
	}
	RespondJSON(w, http.StatusOK, map[string]interface{}{"data": obj})
}
