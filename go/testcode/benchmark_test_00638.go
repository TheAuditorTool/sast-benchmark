package testcode

import (
	"net/http"
	"strconv"
)

func BenchmarkTest00638(w http.ResponseWriter, r *http.Request) {
	if err := r.ParseForm(); err != nil {
		http.Error(w, "invalid form data", http.StatusBadRequest)
		return
	}

	priceStr := r.FormValue("price")
	qtyStr := r.FormValue("quantity")

	p, _ := strconv.ParseFloat(priceStr, 64)
	qty, _ := strconv.Atoi(qtyStr)

	total := p * float64(qty)

	_, err := DB.Exec(
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
