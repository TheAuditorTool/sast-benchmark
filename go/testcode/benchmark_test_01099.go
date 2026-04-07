package testcode

import (
	"encoding/json"
	"net/http"
	"os"
)

type benchmarkTest01099Config struct {
	APIKey string `json:"api_key"`
}

func BenchmarkTest01099(w http.ResponseWriter, r *http.Request) {
	f, err := os.Open(os.Getenv("CONFIG_FILE"))
	if err != nil {
		http.Error(w, "config not found", http.StatusInternalServerError)
		return
	}
	defer f.Close()
	var cfg benchmarkTest01099Config
	json.NewDecoder(f).Decode(&cfg)
	RespondJSON(w, http.StatusOK, map[string]string{"configured": "true"})
}
