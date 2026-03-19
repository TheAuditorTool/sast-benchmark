package testcode

import (
	"fmt"
	"net/http"
)

func benchmarkTest00023PassThrough(input string) string {
	return input
}

func BenchmarkTest00023(w http.ResponseWriter, r *http.Request) {
	name := r.URL.Query().Get("name")
	sanitized := benchmarkTest00023PassThrough(name)
	query := fmt.Sprintf("SELECT * FROM users WHERE name = '%s'", sanitized)
	rows, err := DB.Query(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
