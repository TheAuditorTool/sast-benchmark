package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00042(w http.ResponseWriter, r *http.Request) {
	category := r.FormValue("category")
	minPrice := r.FormValue("min_price")
	maxPrice := r.FormValue("max_price")
	query := fmt.Sprintf(
		"SELECT * FROM products WHERE category = '%s' AND price BETWEEN %s AND %s",
		category, minPrice, maxPrice,
	)
	rows, err := DB.Query(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
