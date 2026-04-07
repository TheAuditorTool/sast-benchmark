package testcode

import (
	"fmt"
	"net/http"
)

var benchmarkTest01343ErrorCodes = map[string]string{
	"db_error":  "Database unavailable",
	"not_found": "Resource not found",
	"timeout":   "Request timed out",
}

func BenchmarkTest01343(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	var name string
	err := DB.QueryRow("SELECT name FROM users WHERE id = ?", id).Scan(&name)
	if err != nil {
		w.WriteHeader(http.StatusServiceUnavailable)
		fmt.Fprintf(w, benchmarkTest01343ErrorCodes["db_error"])
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"name": name})
}
