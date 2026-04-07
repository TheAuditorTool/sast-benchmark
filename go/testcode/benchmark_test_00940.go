package testcode

import (
	"encoding/gob"
	"io"
	"net/http"
	"reflect"
)

func benchmarkTest00940Decode(r io.Reader) interface{} {
	var v interface{}
	gob.NewDecoder(r).Decode(&v)
	return v
}

func BenchmarkTest00940(w http.ResponseWriter, r *http.Request) {
	obj := benchmarkTest00940Decode(r.Body)
	t := reflect.TypeOf(obj)
	var typeName string
	if t != nil {
		typeName = t.String()
	}
	RespondJSON(w, http.StatusOK, map[string]interface{}{"type": typeName, "data": obj})
}
