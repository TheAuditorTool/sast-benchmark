package testcode

import (
	"net/http"
)

type benchmarkTest00686Config struct {
	Host       string `json:"host"`
	Port       int    `json:"port"`
	DBPassword string `json:"-"`
	APIKey     string `json:"-"`
	JWTSecret  string `json:"-"`
}

var benchmarkTest00686AppConfig = benchmarkTest00686Config{
	Host:       "db.internal.example.com",
	Port:       5432,
	DBPassword: "s3cur3P@ssw0rd!",
	APIKey:     "sk-prod-a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6",
	JWTSecret:  "hs256-jwt-secret-do-not-share-xK9mP2qR",
}

func BenchmarkTest00686(w http.ResponseWriter, r *http.Request) {
	format := r.URL.Query().Get("format")
	if format == "" {
		format = "json"
	}

	RespondJSON(w, http.StatusOK, benchmarkTest00686AppConfig)
}
