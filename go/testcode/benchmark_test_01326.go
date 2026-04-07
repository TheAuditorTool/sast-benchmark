package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest01326(w http.ResponseWriter, r *http.Request) {
	table := r.URL.Query().Get("table")
	query := "SELECT id, name FROM " + table + " WHERE active = 1"
	_, err := DB.Exec(query)
	if err != nil {
		fmt.Fprintf(w, "failed: "+query)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
