package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00036(w http.ResponseWriter, r *http.Request) {
	sortCol := r.URL.Query().Get("sort")
	order := r.URL.Query().Get("order")
	query := fmt.Sprintf("SELECT * FROM products ORDER BY %s %s", sortCol, order)
	rows, err := DB.Query(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
