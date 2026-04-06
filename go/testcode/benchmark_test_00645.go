package testcode

import (
	"net/http"
)

var benchmarkTest00645AllowedSorts = map[string]bool{
	"asc":  true,
	"desc": true,
}

func BenchmarkTest00645(w http.ResponseWriter, r *http.Request) {
	sortOrder := r.URL.Query().Get("sort_order")

	if !benchmarkTest00645AllowedSorts[sortOrder] {
		sortOrder = "asc"
	}

	query := "SELECT id, name, score FROM leaderboard ORDER BY score " + sortOrder + " LIMIT 50"

	rows, err := DB.Query(query)
	if err != nil {
		http.Error(w, "database error", http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	entries := []map[string]interface{}{}
	for rows.Next() {
		var id int
		var name string
		var score float64
		if err := rows.Scan(&id, &name, &score); err != nil {
			continue
		}
		entries = append(entries, map[string]interface{}{
			"id":    id,
			"name":  name,
			"score": score,
		})
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"sort_order": sortOrder,
		"entries":    entries,
		"count":      len(entries),
	})
}
