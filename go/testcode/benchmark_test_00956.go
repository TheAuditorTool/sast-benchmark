package testcode

import (
	"encoding/json"
	"io"
	"net/http"
	"reflect"
)

type benchmarkTest00956Record struct {
	ID   int    `json:"id"`
	Name string `json:"name"`
}

func BenchmarkTest00956(w http.ResponseWriter, r *http.Request) {
	body, _ := io.ReadAll(r.Body)
	var rec benchmarkTest00956Record
	json.Unmarshal(body, &rec)
	v := reflect.ValueOf(rec)
	fields := make(map[string]interface{})
	t := v.Type()
	for i := 0; i < v.NumField(); i++ {
		fields[t.Field(i).Name] = v.Field(i).Interface()
	}
	RespondJSON(w, http.StatusOK, fields)
}
