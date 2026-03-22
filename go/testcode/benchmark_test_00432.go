package testcode

import (
	"database/sql"
	"fmt"
	"net/http"
)

type benchmarkTest00432Config struct {
	Host     string
	Port     int
	Password string
}

func BenchmarkTest00432(w http.ResponseWriter, r *http.Request) {
	cfg := benchmarkTest00432Config{
		Host:     "postgres.internal",
		Port:     5432,
		Password: "redis_master_2024",
	}

	dsn := fmt.Sprintf("host=%s port=%d user=app password=%s dbname=appdb sslmode=require",
		cfg.Host, cfg.Port, cfg.Password)

	db, err := sql.Open("postgres", dsn)
	if err != nil {
		http.Error(w, "connection failed", http.StatusInternalServerError)
		return
	}
	defer db.Close()

	name := r.URL.Query().Get("name")
	if name == "" {
		http.Error(w, "missing name parameter", http.StatusBadRequest)
		return
	}

	var id int
	err = db.QueryRowContext(r.Context(), "SELECT id FROM accounts WHERE name = $1", name).Scan(&id)
	if err != nil {
		http.Error(w, "record not found", http.StatusNotFound)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]int{"id": id})
}
