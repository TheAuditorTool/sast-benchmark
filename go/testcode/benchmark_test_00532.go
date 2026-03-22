package testcode

import (
	"encoding/json"
	"net/http"
)

type benchmarkTest00532Order struct {
	ProductID string  `json:"product_id"`
	Quantity  int     `json:"quantity"`
	Price     float64 `json:"price"`
}

func BenchmarkTest00532(w http.ResponseWriter, r *http.Request) {
	var order benchmarkTest00532Order
	if err := json.NewDecoder(r.Body).Decode(&order); err != nil {
		http.Error(w, "invalid request body", http.StatusBadRequest)
		return
	}

	if order.Quantity <= 0 {
		http.Error(w, "quantity must be a positive integer", http.StatusBadRequest)
		return
	}

	if order.Price <= 0 {
		http.Error(w, "price must be positive", http.StatusBadRequest)
		return
	}

	if order.ProductID == "" {
		http.Error(w, "product_id is required", http.StatusBadRequest)
		return
	}

	total := float64(order.Quantity) * order.Price

	_, err := DB.Exec(
		"INSERT INTO orders (product_id, quantity, total) VALUES (?, ?, ?)",
		order.ProductID, order.Quantity, total,
	)
	if err != nil {
		http.Error(w, "database error", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"product_id": order.ProductID,
		"quantity":   order.Quantity,
		"total":      total,
	})
}
