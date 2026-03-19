package testcode

import (
	"fmt"
	"net/http"
	"strings"
)

func benchmarkTest00022Sanitize(input string) string {
	return strings.Replace(input, "'", "", -1)
}

func BenchmarkTest00022(w http.ResponseWriter, r *http.Request) {
	name := r.URL.Query().Get("name")
	cleaned := benchmarkTest00022Sanitize(name)
	query := fmt.Sprintf("SELECT * FROM users WHERE name = '%s'", cleaned)
	rows, err := DB.Query(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
