package testcode

import (
	"encoding/xml"
	"io"
	"net/http"
)

type benchmarkTest00961Product struct {
	XMLName xml.Name `xml:"product"`
	SKU     string   `xml:"sku"`
	Price   float64  `xml:"price"`
}

func BenchmarkTest00961(w http.ResponseWriter, r *http.Request) {
	body, err := io.ReadAll(io.LimitReader(r.Body, 1<<16))
	if err != nil {
		http.Error(w, "read error", http.StatusBadRequest)
		return
	}
	var p benchmarkTest00961Product
	if err := xml.Unmarshal(body, &p); err != nil {
		http.Error(w, "xml error", http.StatusBadRequest)
		return
	}
	if p.Price < 0 {
		http.Error(w, "invalid price", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]interface{}{"sku": p.SKU, "price": p.Price})
}
