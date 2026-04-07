package testcode

import (
	"encoding/xml"
	"io"
	"net/http"
)

func BenchmarkTest00934(w http.ResponseWriter, r *http.Request) {
	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "read error", http.StatusBadRequest)
		return
	}
	var data interface{}
	if err := xml.Unmarshal(body, &data); err != nil {
		http.Error(w, "xml error", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]interface{}{"data": data})
}
