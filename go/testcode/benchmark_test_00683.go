package testcode

import (
	"log/slog"
	"net/http"

	"github.com/google/uuid"
)

func BenchmarkTest00683(w http.ResponseWriter, r *http.Request) {
	queryID := uuid.New().String()

	sku := r.URL.Query().Get("sku")
	if sku == "" {
		http.Error(w, "sku required", http.StatusBadRequest)
		return
	}

	var productID int
	var price float64
	err := DB.QueryRowContext(r.Context(),
		"SELECT id, price FROM products WHERE sku = ?", sku,
	).Scan(&productID, &price)
	if err != nil {
		slog.Error("query failed", "error", err, "query_id", queryID)
		http.Error(w, "database error", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"product_id": productID,
		"sku":        sku,
		"price":      price,
	})
}
