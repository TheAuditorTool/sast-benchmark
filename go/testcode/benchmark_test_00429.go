package testcode

import (
	"database/sql"
	"net/http"
)

func BenchmarkTest00429(w http.ResponseWriter, r *http.Request) {
	db, err := sql.Open("postgres", "host=db.internal port=5432 user=admin password=s3cr3tP4ss dbname=production sslmode=disable")
	if err != nil {
		http.Error(w, "database error", http.StatusInternalServerError)
		return
	}
	defer db.Close()
	var count int
	db.QueryRow("SELECT COUNT(*) FROM users").Scan(&count)
	RespondJSON(w, http.StatusOK, map[string]int{"users": count})
}
