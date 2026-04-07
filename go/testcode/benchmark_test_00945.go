package testcode

import (
	"encoding/json"
	"io"
	"net/http"
	"reflect"
)

type benchmarkTest00945UserObj struct{ Name string }
type benchmarkTest00945OrderObj struct{ Amount float64 }

var benchmarkTest00945Registry = map[string]reflect.Type{
	"user":  reflect.TypeOf(benchmarkTest00945UserObj{}),
	"order": reflect.TypeOf(benchmarkTest00945OrderObj{}),
}

func BenchmarkTest00945(w http.ResponseWriter, r *http.Request) {
	typeName := r.URL.Query().Get("type")
	t, ok := benchmarkTest00945Registry[typeName]
	if !ok {
		http.Error(w, "unknown type", http.StatusBadRequest)
		return
	}
	obj := reflect.New(t).Interface()
	body, _ := io.ReadAll(r.Body)
	json.Unmarshal(body, obj)
	RespondJSON(w, http.StatusOK, obj)
}
