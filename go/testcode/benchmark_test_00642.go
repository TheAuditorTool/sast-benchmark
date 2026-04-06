package testcode

import (
	"net/http"
	"strconv"
)

func BenchmarkTest00642(w http.ResponseWriter, r *http.Request) {
	capStr := r.URL.Query().Get("capacity")
	if capStr == "" {
		http.Error(w, "capacity parameter required", http.StatusBadRequest)
		return
	}

	cap, _ := strconv.Atoi(capStr)

	s := make([]string, 0, cap)

	rows, err := DB.Query("SELECT name FROM tags LIMIT ?", cap)
	if err != nil {
		http.Error(w, "database error", http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	for rows.Next() {
		var name string
		if err := rows.Scan(&name); err != nil {
			continue
		}
		s = append(s, name)
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"capacity": cap,
		"tags":     s,
		"count":    len(s),
	})
}
