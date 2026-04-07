package testcode

import (
	"net/http"
	"os"
)

type benchmarkTest01101ServiceConfig struct {
	DBPassword string
	APIKey     string
	JWTSecret  string
}

func benchmarkTest01101LoadConfig() benchmarkTest01101ServiceConfig {
	return benchmarkTest01101ServiceConfig{
		DBPassword: os.Getenv("DB_PASSWORD"),
		APIKey:     os.Getenv("SERVICE_API_KEY"),
		JWTSecret:  os.Getenv("JWT_SECRET"),
	}
}

func BenchmarkTest01101(w http.ResponseWriter, r *http.Request) {
	cfg := benchmarkTest01101LoadConfig()
	RespondJSON(w, http.StatusOK, map[string]bool{
		"db_configured":  cfg.DBPassword != "",
		"api_configured": cfg.APIKey != "",
	})
}
