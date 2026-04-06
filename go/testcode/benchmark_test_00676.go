package testcode

import (
	"net/http"
)

var benchmarkTest00676Config = struct {
	Host       string
	Port       int
	DBPassword string
	APIKey     string
	JWTSecret  string
}{
	Host:       "db.internal.example.com",
	Port:       5432,
	DBPassword: "s3cur3P@ssw0rd!",
	APIKey:     "sk-prod-a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6",
	JWTSecret:  "hs256-jwt-secret-do-not-share-xK9mP2qR",
}

func BenchmarkTest00676(w http.ResponseWriter, r *http.Request) {
	format := r.URL.Query().Get("format")
	if format == "" {
		format = "json"
	}

	RespondJSON(w, http.StatusOK, benchmarkTest00676Config)
}
