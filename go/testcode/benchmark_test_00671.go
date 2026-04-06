package testcode

import (
	"net/http"
	_ "net/http/pprof"
)

func BenchmarkTest00671(w http.ResponseWriter, r *http.Request) {
	category := r.URL.Query().Get("category")
	if category == "" {
		http.Error(w, "category required", http.StatusBadRequest)
		return
	}

	rows, err := DB.QueryContext(r.Context(),
		"SELECT id, name, price FROM products WHERE category = ?", category,
	)
	if err != nil {
		http.Error(w, "internal server error", http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	type product struct {
		ID    int     `json:"id"`
		Name  string  `json:"name"`
		Price float64 `json:"price"`
	}
	var products []product
	for rows.Next() {
		var p product
		if err := rows.Scan(&p.ID, &p.Name, &p.Price); err != nil {
			http.Error(w, "internal server error", http.StatusInternalServerError)
			return
		}
		products = append(products, p)
	}

	RespondJSON(w, http.StatusOK, products)
}
