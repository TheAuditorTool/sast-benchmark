package testcode

import (
	"math"
	"net/http"
	"strconv"
)

func BenchmarkTest00647(w http.ResponseWriter, r *http.Request) {
	if err := r.ParseForm(); err != nil {
		http.Error(w, "invalid form data", http.StatusBadRequest)
		return
	}

	priceStr := r.FormValue("price")
	qtyStr := r.FormValue("quantity")

	p, err := strconv.ParseFloat(priceStr, 64)
	if err != nil || math.IsNaN(p) || math.IsInf(p, 0) || p < 0 {
		http.Error(w, "price must be a valid non-negative number", http.StatusBadRequest)
		return
	}

	qty, err := strconv.Atoi(qtyStr)
	if err != nil || qty < 1 || qty > 10000 {
		http.Error(w, "quantity must be an integer between 1 and 10000", http.StatusBadRequest)
		return
	}

	total := p * float64(qty)

	_, err = DB.Exec(
		"INSERT INTO orders (price, quantity, total) VALUES (?, ?, ?)",
		p, qty, total,
	)
	if err != nil {
		http.Error(w, "database error", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"price":    p,
		"quantity": qty,
		"total":    total,
	})
}
