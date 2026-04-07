package testcode

import (
	"encoding/json"
	"io"
	"net/http"
	"reflect"
)

var benchmarkTest00935Types = map[string]reflect.Type{}

func BenchmarkTest00935(w http.ResponseWriter, r *http.Request) {
	body, _ := io.ReadAll(r.Body)
	var envelope map[string]interface{}
	json.Unmarshal(body, &envelope)
	typeName, _ := envelope["type"].(string)
	t, ok := benchmarkTest00935Types[typeName]
	if !ok {
		http.Error(w, "unknown type", http.StatusBadRequest)
		return
	}
	obj := reflect.New(t).Interface()
	json.Unmarshal(body, obj)
	RespondJSON(w, http.StatusOK, obj)
}
