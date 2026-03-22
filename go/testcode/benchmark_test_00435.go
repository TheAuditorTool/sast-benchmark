package testcode

import (
	"database/sql"
	"fmt"
	"net/http"
	"os"
)

func BenchmarkTest00435(w http.ResponseWriter, r *http.Request) {
	password := os.Getenv("DB_PASSWORD")
	if password == "" {
		http.Error(w, "database configuration error", http.StatusInternalServerError)
		return
	}

	host := os.Getenv("DB_HOST")
	if host == "" {
		host = "localhost"
	}

	dsn := fmt.Sprintf("host=%s port=5432 user=appuser password=%s dbname=appdb sslmode=require", host, password)
	db, err := sql.Open("postgres", dsn)
	if err != nil {
		http.Error(w, "connection failed", http.StatusInternalServerError)
		return
	}
	defer db.Close()

	var count int
	if err := db.QueryRowContext(r.Context(), "SELECT COUNT(*) FROM sessions WHERE active = true").Scan(&count); err != nil {
		http.Error(w, "query failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]int{"active_sessions": count})
}
