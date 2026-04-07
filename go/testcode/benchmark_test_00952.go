package testcode

import (
	"encoding/xml"
	"io"
	"net/http"
)

type benchmarkTest00952Item struct {
	XMLName xml.Name `xml:"item"`
	ID      int      `xml:"id"`
	Name    string   `xml:"name"`
}

func BenchmarkTest00952(w http.ResponseWriter, r *http.Request) {
	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "read error", http.StatusBadRequest)
		return
	}
	var item benchmarkTest00952Item
	if err := xml.Unmarshal(body, &item); err != nil {
		http.Error(w, "xml error", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]interface{}{"id": item.ID, "name": item.Name})
}
