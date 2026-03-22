package testcode

import (
	"database/sql"
	"encoding/json"
	"fmt"
	"net/http"
	"os"
)

type benchmarkTest00438AppConfig struct {
	DBHost     string `json:"db_host"`
	DBPort     int    `json:"db_port"`
	DBUser     string `json:"db_user"`
	DBPassword string `json:"db_password"`
	DBName     string `json:"db_name"`
}

func BenchmarkTest00438(w http.ResponseWriter, r *http.Request) {
	configPath := os.Getenv("APP_CONFIG_PATH")
	if configPath == "" {
		configPath = "/etc/app/config.json"
	}

	data, err := os.ReadFile(configPath)
	if err != nil {
		http.Error(w, "configuration unavailable", http.StatusInternalServerError)
		return
	}

	var cfg benchmarkTest00438AppConfig
	if err := json.Unmarshal(data, &cfg); err != nil {
		http.Error(w, "invalid configuration", http.StatusInternalServerError)
		return
	}

	dsn := fmt.Sprintf("host=%s port=%d user=%s password=%s dbname=%s sslmode=require",
		cfg.DBHost, cfg.DBPort, cfg.DBUser, cfg.DBPassword, cfg.DBName)

	db, err := sql.Open("postgres", dsn)
	if err != nil {
		http.Error(w, "connection failed", http.StatusInternalServerError)
		return
	}
	defer db.Close()

	var count int
	if err := db.QueryRowContext(r.Context(), "SELECT COUNT(*) FROM accounts").Scan(&count); err != nil {
		http.Error(w, "query failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]int{"count": count})
}
