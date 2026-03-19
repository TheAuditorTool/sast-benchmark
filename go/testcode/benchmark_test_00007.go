package testcode

import (
	"fmt"
	"net/http"
	"strings"
)

func BenchmarkTest00007(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()
	columns := r.Form["col"]
	table := r.FormValue("table")
	cols := strings.Join(columns, ", ")
	query := fmt.Sprintf("SELECT %s FROM %s", cols, table)
	rows, err := DB.Query(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
