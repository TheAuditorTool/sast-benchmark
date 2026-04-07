package testcode

import (
	"encoding/json"
	"net/http"
)

type benchmarkTest00957Order struct {
	ProductID int     `json:"product_id"`
	Quantity  int     `json:"quantity"`
	Price     float64 `json:"price"`
}

func BenchmarkTest00957(w http.ResponseWriter, r *http.Request) {
	r.Body = http.MaxBytesReader(w, r.Body, 1<<20)
	var order benchmarkTest00957Order
	if err := json.NewDecoder(r.Body).Decode(&order); err != nil {
		http.Error(w, "json error", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"product": order.ProductID,
		"qty":     order.Quantity,
	})
}
