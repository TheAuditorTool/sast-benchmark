package testcode

import (
	"net/http"
)

var benchmarkTest00031Tables = map[string]bool{
	"users":    true,
	"products": true,
	"orders":   true,
}

func BenchmarkTest00031(w http.ResponseWriter, r *http.Request) {
	table := r.URL.Query().Get("table")
	if !benchmarkTest00031Tables[table] {
		http.Error(w, "invalid table", http.StatusBadRequest)
		return
	}
	rows, err := DB.Query("SELECT COUNT(*) FROM " + table)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
