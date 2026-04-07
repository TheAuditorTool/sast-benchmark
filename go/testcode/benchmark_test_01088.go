package testcode

import "net/http"

type benchmarkTest01088DBConfig struct {
	Host     string
	Port     int
	DBName   string
	Password string
}

var benchmarkTest01088Config = benchmarkTest01088DBConfig{
	Host:     "db.example.com",
	Port:     5432,
	DBName:   "production",
	Password: "prod_secret_123",
}

func BenchmarkTest01088(w http.ResponseWriter, r *http.Request) {
	RespondJSON(w, http.StatusOK, map[string]string{"db": benchmarkTest01088Config.Host})
}
