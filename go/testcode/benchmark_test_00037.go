package testcode

import (
	"net/http"
)

var benchmarkTest00037SortCols = map[string]string{
	"name":  "name",
	"price": "price",
	"date":  "created_at",
}

func BenchmarkTest00037(w http.ResponseWriter, r *http.Request) {
	sortKey := r.URL.Query().Get("sort")
	col, ok := benchmarkTest00037SortCols[sortKey]
	if !ok {
		col = "id"
	}
	rows, err := DB.Query("SELECT * FROM products ORDER BY " + col)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
