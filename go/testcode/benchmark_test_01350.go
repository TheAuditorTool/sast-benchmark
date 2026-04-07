package testcode

import (
	"net/http"
)

type benchmarkTest01350AppConfig struct {
	AppName  string `json:"app_name"`
	Version  string `json:"version"`
	DBHost   string `json:"-"`
	DBPass   string `json:"-"`
	SecretKey string `json:"-"`
}

var benchmarkTest01350Config = benchmarkTest01350AppConfig{
	AppName:   "myapp",
	Version:   "1.0.0",
	DBHost:    "db.internal:5432",
	DBPass:    "s3cr3t",
	SecretKey: "jwt-secret-key",
}

func BenchmarkTest01350(w http.ResponseWriter, r *http.Request) {
	RespondJSON(w, http.StatusOK, benchmarkTest01350Config)
}
