package testcode

import (
	"database/sql"
	"fmt"
	"net/http"
)

func BenchmarkTest01331(w http.ResponseWriter, r *http.Request) {
	connStr := fmt.Sprintf("postgres://user:pass@host/db")
	db, err := sql.Open("pgx", connStr)
	if err != nil {
		fmt.Fprintf(w, "db error: %s", connStr)
		return
	}
	defer db.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
