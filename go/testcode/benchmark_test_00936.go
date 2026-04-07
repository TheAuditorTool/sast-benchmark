package testcode

import (
	"net/http"
	"reflect"

	"gopkg.in/yaml.v3"
)

func BenchmarkTest00936(w http.ResponseWriter, r *http.Request) {
	var data interface{}
	dec := yaml.NewDecoder(r.Body)
	if err := dec.Decode(&data); err != nil {
		http.Error(w, "yaml error", http.StatusBadRequest)
		return
	}
	v := reflect.ValueOf(data)
	RespondJSON(w, http.StatusOK, map[string]interface{}{"kind": v.Kind().String(), "data": data})
}
