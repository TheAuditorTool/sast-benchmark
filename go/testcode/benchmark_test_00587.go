package testcode

import (
	"encoding/xml"
	"io"
	"net/http"
)

type benchmarkTest00587Order struct {
	XMLName  xml.Name `xml:"order"`
	OrderID  string   `xml:"order_id"`
	Product  string   `xml:"product"`
	Quantity int      `xml:"quantity"`
	Total    float64  `xml:"total"`
}

func BenchmarkTest00587(w http.ResponseWriter, r *http.Request) {
	limited := io.LimitReader(r.Body, 1<<20)
	dec := xml.NewDecoder(limited)

	var order benchmarkTest00587Order
	if err := dec.Decode(&order); err != nil {
		http.Error(w, "xml parse error", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, order)
}
