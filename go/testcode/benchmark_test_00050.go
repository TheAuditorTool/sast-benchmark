package testcode

import (
	"fmt"
	"net/http"
)

func benchmarkTest00050BuildWhere(field, value string) string {
	return fmt.Sprintf("%s = '%s'", field, value)
}

func benchmarkTest00050BuildQuery(table string, where string) string {
	return fmt.Sprintf("SELECT * FROM %s WHERE %s", table, where)
}

func BenchmarkTest00050(w http.ResponseWriter, r *http.Request) {
	field := r.URL.Query().Get("field")
	value := r.URL.Query().Get("value")
	where := benchmarkTest00050BuildWhere(field, value)
	query := benchmarkTest00050BuildQuery("users", where)
	rows, err := DB.Query(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
