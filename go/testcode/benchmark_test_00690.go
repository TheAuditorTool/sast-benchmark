package testcode

import (
	"net/http"
)

var benchmarkTest00690AllowedCols = map[string]string{
	"name":       "name",
	"created_at": "created_at",
	"price":      "price",
}

func BenchmarkTest00690(w http.ResponseWriter, r *http.Request) {
	sortParam := r.URL.Query().Get("sort")
	col, ok := benchmarkTest00690AllowedCols[sortParam]
	if !ok {
		col = "created_at"
	}
	rows, err := DB.Query("SELECT id, name FROM products ORDER BY " + col)
	if err != nil {
		http.Error(w, "query error", http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
