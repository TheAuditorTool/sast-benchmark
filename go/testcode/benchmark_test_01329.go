package testcode

import (
	"encoding/json"
	"net/http"
)

type benchmarkTest01329Config struct {
	Host     string `json:"host"`
	Password string `json:"password"`
}

var benchmarkTest01329Cfg = benchmarkTest01329Config{
	Host:     "db.internal",
	Password: "s3cr3t",
}

func BenchmarkTest01329(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(benchmarkTest01329Cfg)
}
