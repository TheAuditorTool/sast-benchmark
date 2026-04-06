package testcode

import (
	"net/http"
	"strconv"
)

const benchmarkTest00644PageSize = 20

func BenchmarkTest00644(w http.ResponseWriter, r *http.Request) {
	pageStr := r.URL.Query().Get("page")

	n, err := strconv.Atoi(pageStr)
	if err != nil || n < 1 || n > 1000 {
		http.Error(w, "page must be an integer between 1 and 1000", http.StatusBadRequest)
		return
	}

	offset := (n - 1) * benchmarkTest00644PageSize

	rows, err := DB.Query(
		"SELECT id, name, created_at FROM items ORDER BY created_at DESC LIMIT ? OFFSET ?",
		benchmarkTest00644PageSize, offset,
	)
	if err != nil {
		http.Error(w, "database error", http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	items := []map[string]interface{}{}
	for rows.Next() {
		var id int
		var name, createdAt string
		if err := rows.Scan(&id, &name, &createdAt); err != nil {
			continue
		}
		items = append(items, map[string]interface{}{
			"id":         id,
			"name":       name,
			"created_at": createdAt,
		})
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"page":      n,
		"page_size": benchmarkTest00644PageSize,
		"items":     items,
	})
}
