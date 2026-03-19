package testcode

import (
	"encoding/json"
	"net/http"
)

type createOrderRequest struct {
	ProductID int    `json:"product_id"`
	Quantity  int    `json:"quantity"`
	Address   string `json:"address"`
	Notes     string `json:"notes"`
}

func BenchmarkTest00256(w http.ResponseWriter, r *http.Request) {
	var req createOrderRequest
	decoder := json.NewDecoder(r.Body)
	err := decoder.Decode(&req)
	if err != nil {
		http.Error(w, "invalid request body", http.StatusBadRequest)
		return
	}

	if req.ProductID <= 0 || req.Quantity <= 0 {
		http.Error(w, "invalid product or quantity", http.StatusBadRequest)
		return
	}

	var price float64
	err = DB.QueryRow("SELECT price FROM products WHERE id = ?", req.ProductID).Scan(&price)
	if err != nil {
		http.Error(w, "product not found", http.StatusNotFound)
		return
	}

	total := price * float64(req.Quantity)
	result, err := DB.Exec("INSERT INTO orders (product_id, quantity, total, address, notes) VALUES (?, ?, ?, ?, ?)",
		req.ProductID, req.Quantity, total, req.Address, req.Notes)
	if err != nil {
		http.Error(w, "order creation failed", http.StatusInternalServerError)
		return
	}

	orderID, _ := result.LastInsertId()
	RespondJSON(w, http.StatusCreated, map[string]interface{}{
		"order_id": orderID,
		"total":    total,
	})
}
