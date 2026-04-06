package testcode

import (
	"bytes"
	"encoding/base64"
	"encoding/binary"
	"net/http"
	"reflect"
)

type benchmarkTest00583TypeA struct {
	ID    int32
	Score float32
}

type benchmarkTest00583TypeB struct {
	Count int64
	Value float64
}

var benchmarkTest00583TypeRegistry = map[string]reflect.Type{
	"TypeA": reflect.TypeOf(benchmarkTest00583TypeA{}),
	"TypeB": reflect.TypeOf(benchmarkTest00583TypeB{}),
}

func BenchmarkTest00583(w http.ResponseWriter, r *http.Request) {
	var body struct {
		TypeName string `json:"type_name"`
		Data     string `json:"data"`
	}
	if err := ParseJSONBody(r, &body); err != nil {
		http.Error(w, "invalid request", http.StatusBadRequest)
		return
	}

	t, ok := benchmarkTest00583TypeRegistry[body.TypeName]
	if !ok {
		http.Error(w, "unknown type", http.StatusBadRequest)
		return
	}

	raw, err := base64.StdEncoding.DecodeString(body.Data)
	if err != nil {
		http.Error(w, "invalid data encoding", http.StatusBadRequest)
		return
	}

	obj := reflect.New(t)
	if err := binary.Read(bytes.NewReader(raw), binary.LittleEndian, obj.Interface()); err != nil {
		http.Error(w, "binary decode error", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, obj.Interface())
}
