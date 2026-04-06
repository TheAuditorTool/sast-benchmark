package testcode

import (
	"encoding/xml"
	"net/http"
)

func BenchmarkTest00582(w http.ResponseWriter, r *http.Request) {
	var data interface{}
	dec := xml.NewDecoder(r.Body)
	if err := dec.Decode(&data); err != nil {
		http.Error(w, "xml decode error", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, data)
}
