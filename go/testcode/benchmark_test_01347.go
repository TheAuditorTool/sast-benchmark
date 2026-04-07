package testcode

import (
	"encoding/json"
	"net/http"
)

type benchmarkTest01347AdminConfig struct {
	Host     string `json:"host"`
	Port     int    `json:"port"`
	Password string `json:"password"`
	APIKey   string `json:"api_key"`
}

var benchmarkTest01347Cfg = benchmarkTest01347AdminConfig{
	Host:     "db.internal",
	Port:     5432,
	Password: "s3cr3t",
	APIKey:   "key-abc123",
}

func BenchmarkTest01347(w http.ResponseWriter, r *http.Request) {
	cfg := benchmarkTest01347Cfg
	cfg.Password = "***"
	cfg.APIKey = "***"
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(cfg)
}
